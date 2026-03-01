use crate::{dsp::{correlation:: perform_self_correlation, stft::FftContext, wavelet::{analyze_wavelet}}, models::{self, CorrelationSettings}, results::TransientResults, utils::{find_nearest_zero, find_peak_sample, find_previous_zero, closest_power_of_usize}};

pub fn process(unit: &models::AudioInfo, settings: &models::TransientSettings, pitch: f32, context: &mut FftContext) -> TransientResults
{
    //Initialize values
    let mut t_results: TransientResults = Default::default();
    let rms_size: usize;
    if settings.use_ms_size{
        rms_size = (settings.rms_ms_size * unit.sample_rate / 1000.0) as usize;
    }
    else {
        rms_size = settings.rms_sample_size;
    }

    //TRANSIENT RANGE
    t_results.range.init = find_init(unit, settings, rms_size);
    t_results.range.end = find_end(unit, settings, pitch, t_results.range.init);
    t_results.t_length = t_results.range.end - t_results.range.init;


    //SCALOGRAM

    //Setup and initial calculations
    let t_with_offset: usize = t_results.range.end + settings.wavelet_settings.tail_off;    
    t_results.scalogram_length = closest_power_of_usize(t_with_offset - t_results.range.init, 2);
    let scalogram_end: usize = t_results.range.init + t_results.scalogram_length;
    t_results.scalo_range = crate::structures::SampleRange { init: t_results.range.init, end: scalogram_end };

    //Scalogram analysis with Wavelet Transform
    t_results.scalogram = analyze_wavelet(
        &unit.audio_file[t_results.range.init..scalogram_end], 
        settings.wavelet_settings.clone(), 
        t_results.scalogram_length,
        unit, 
        context);

    //GAIN AND RMS
    let mut source_sum_sq: f32 = 0.0;
    if t_results.scalogram_length > 0{
        for i in 0..t_results.scalogram_length{
            let sample = unit.audio_file[t_results.scalo_range.init + i];
            source_sum_sq += sample * sample;
        }
    }
    let source_rms: f32;
    if t_results.scalogram_length > 0 {
        source_rms = (source_sum_sq / t_results.scalogram_length as f32).sqrt();
    }
    else {source_rms = 0.0;}

    let mut model_total_power: f32 = 0.0;
    for partial in t_results.scalogram.as_slice(){
        if partial.data.is_empty() {continue;}
        let mut partial_sum_sq: f32 = 0.0;
        for val in partial.data.as_slice(){
            partial_sum_sq += val * val;
        }

        let partial_mean_sq = partial_sum_sq / partial.data.len() as f32;

        let partial_power = partial_mean_sq / 2.0;

        model_total_power += partial_power;
    }

    let model_rms = model_total_power.sqrt();
    let gain: f32;
    if model_rms > 0.00001 {
        gain = source_rms / model_rms;
    }
    else {gain = 1.0;}

    for partial in &mut t_results.scalogram{
        for val in &mut partial.data{
            *val *= gain;
        }
    }

    let mut sum = 0.0;
    for i in t_results.range.init..t_results.scalo_range.end{
        sum += unit.audio_file[i] * unit.audio_file[i];
    }
    t_results.rms = (sum / t_results.scalogram_length as f32).sqrt();

    return t_results;
}

fn find_init(unit: &models::AudioInfo, settings: &models::TransientSettings, rms_size: usize) -> usize
{
    //Initialize, declare and all that stuff
    let start_sample: usize = settings.start_sample;
    let fact: f64 = settings.rms_factor;
    let thresh: f64 = settings.rms_threshold;
    let mut rms_list: Vec<f64> = Vec::new();
    let mut found: bool = false;
    let max_samp: usize = unit.audio_file.len() / 4;
    let mut t_init_sample: usize = 0;
    let mut samp = start_sample;

    while samp < max_samp
    {
        let mut sum: f64 = 0.0;
        let mut temp_rms: Vec<f64> = vec![0.0; rms_size];

        for rms_samp in 0..rms_size
        {
            if (rms_samp + samp) >= unit.audio_file.len() {break;}
            let x: f64 = unit.audio_file[rms_samp + samp] as f64;
            sum += x * x;
            temp_rms[rms_samp] = x;
        }

        let div = sum / rms_size as f64;
        let rms = div.sqrt();
        if rms_list.len() < 2 {
            rms_list.push(rms);
            continue;
        }

        let rms_ratio;
        let last_val: f64 = *rms_list.last().unwrap_or(&0.0);
        if last_val.abs() < 1.0e-9
        {
            rms_ratio = 1.0;
        }
        else
        {
            rms_ratio = rms / rms_list.last().unwrap();
        }

        rms_list.push(rms);

        if rms_ratio > fact && rms > thresh
        {
            found = true;
            t_init_sample = samp;
            break;
        }

        samp += settings.hop_size;
    }

    if !found {
        let min_end = unit.audio_file.len().min(10000);
        let peak_samp = find_peak_sample(&unit.audio_file, 0, min_end, true);
        let zero_samp = find_previous_zero(&unit.audio_file, peak_samp);
        return zero_samp;
    }

    let zero_samp = find_previous_zero(&unit.audio_file, t_init_sample);
    return zero_samp;
}

fn find_end(unit: &models::AudioInfo, settings: &models::TransientSettings, pitch: f32, t_init: usize) -> usize
{
    let expected_period: usize = (unit.sample_rate / pitch) as usize;
    let jump: usize = expected_period - (expected_period as f32 * 0.25) as usize;
    let start_sample = t_init + settings.correlation_offset;
    let stop_sample = t_init;

    let peak_sample = find_peak_sample(
        &unit.audio_file, 
        start_sample, 
        start_sample + expected_period * 2, 
        true
    );

    //Find samples
    let corr_start = find_previous_zero(&unit.audio_file, peak_sample);
    let first_w_end: usize = find_nearest_zero(&unit.audio_file, corr_start + expected_period);

    let window_size = first_w_end - corr_start;

    let corr_settings: CorrelationSettings = CorrelationSettings { 
        window_size: window_size, 
        start_sample: start_sample, 
        stop_sample: stop_sample, 
        go_left: true, 
        jump_post_peak: true, 
        jump_size: jump, 
        use_filter: true, 
        is_low_pass: true, 
        threshold: settings.correlation_threshold,
        cutoff: pitch * settings.pitch_multiplier 
    };
        
    let correlation_results = perform_self_correlation(unit, &corr_settings);

    return correlation_results.index_list[0];
}

