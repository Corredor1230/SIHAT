use crate::{dsp::correlation:: perform_self_correlation, models::{self, CorrelationSettings}, results::TransientResults, structures::SampleValueList, utils::{find_peak_sample, find_previous_zero}};

pub fn process(unit: &models::AudioInfo, settings: &models::TransientSettings, pitch: f32) -> TransientResults
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

    //Find the initial sample
    t_results.range.init = find_init(unit, settings, rms_size);
    t_results.range.end = find_end(unit, settings, pitch, t_results.range.init);

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

    let corr_settings: CorrelationSettings = CorrelationSettings { 
        window_size: expected_period, 
        start_sample: t_init + settings.correlation_offset, 
        stop_sample: t_init, 
        go_left: true, 
        jump_post_peak: true, 
        jump_size: jump, 
        use_filter: true, 
        is_low_pass: true, 
        threshold: 0.8,
        cutoff: pitch * 6.0 
    };
        
    let correlation_results = perform_self_correlation(unit, pitch, &corr_settings);

    return correlation_results.index_list[0];
}

