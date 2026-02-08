use crate::{dsp::filter::filtfilt, models::{AudioInfo, CorrelationSettings}, structures::SampleValueList, utils::{find_nearest_zero, find_peak_sample, find_previous_zero}};

pub fn perform_self_correlation(unit: &AudioInfo, pitch: f32, settings: &CorrelationSettings) -> SampleValueList
{
    //Initialize
    let sr = unit.sample_rate;
    let mut index_list: Vec<usize> = Vec::new();
    let mut value_list: Vec<f32> = Vec::new();
    let expected_period: usize = (sr / pitch) as usize;
    let sliding_size: isize = if settings.jump_post_peak {settings.jump_size as isize} else if settings.go_left {-1} else {1};

    let peak_sample = find_peak_sample(
        &unit.audio_file, 
        settings.start_sample, 
        settings.start_sample + expected_period * 2, 
        true
    );

    //Find samples
    let corr_start = find_previous_zero(&unit.audio_file, peak_sample);
    let first_w_end: usize = find_nearest_zero(&unit.audio_file, corr_start + expected_period);

    // Initial Window Setup
    let window_len = first_w_end - corr_start;
    let mut window: Vec<f32> = unit.audio_file[corr_start..first_w_end].to_vec();
    let mut comparative: Vec<f32> = unit.audio_file[corr_start..first_w_end].to_vec();

    if settings.use_filter {
        filtfilt(&mut window, sr, settings.cutoff);
    }

    // Pre-calculate window energy
    let mut square_a: f32 = window.iter().map(|&w| w * w).sum();

    //Initialize more
    let mut corr_list: Vec<f32> = Vec::new();
    let mut earliest_correlation: usize;
    
    // Boxed Iterator 
    let mut cursor = (corr_start - settings.jump_size) as isize;
    let end_limit = settings.stop_sample as isize;

    let step: isize = if cursor < end_limit && !settings.go_left { 1 } else { -1 };

    while (step > 0 && cursor < end_limit) || (step < 0 && cursor > end_limit) {
        let samp = cursor as usize;
        for i in 0..comparative.len() {
            comparative[i] = unit.audio_file[samp + i];
        }
        filtfilt(&mut comparative, sr, settings.cutoff);

        let mut numerator = 0.0;
        let mut square_b = 0.0;

        for i in 0..window_len{
            let a = window[i];
            let b = comparative[i];
            numerator += a * b;
            square_b += b * b;
        }

        let pre_den = square_a * square_b;
        let den = pre_den.sqrt();

        let corr_value;
        if den == 0.0 {corr_value = 0.0;}
        else {corr_value = numerator / den;}

        corr_list.push(corr_value);

        if corr_list.len() < 3 {continue;}

        let len = corr_list.len();
        
        let newest = corr_list[len - 1];
        let target = corr_list[len - 2];
        let oldest = corr_list[len - 3]; 

        let above_thresh = target > settings.threshold;
        let is_peak = (target > newest) && (target > oldest);

        if above_thresh && is_peak{
            earliest_correlation = find_nearest_zero(&unit.audio_file, samp);

            index_list.push(earliest_correlation);
            value_list.push(corr_value);

            //Rewrite window and process...
            for i in 0..window_len{
                window[i] = unit.audio_file[earliest_correlation + i];
            }
            filtfilt(&mut window, sr, settings.cutoff);

            //Recalculate square_a
            square_a = window.iter().map(|&w| w * w).sum();
            cursor += step * (sliding_size as isize);
        }
        else {cursor += step;}
    }

    if settings.go_left{
        index_list.reverse();
        value_list.reverse();
    }
    let sample_list: SampleValueList = SampleValueList { index_list, value_list };
    return sample_list;
}