use crate::{models::{AudioInfo, CorrelationSettings}, structures::SampleValueList, utils::{find_peak_sample, find_previous_zero}};

pub fn perform_cross_correlation(unit: &AudioInfo, pitch: f32, settings: &CorrelationSettings) -> SampleValueList
{
    //Initialize
    let sr = unit.sample_rate;
    let mut index_list: Vec<usize> = Vec::new();
    let mut value_list: Vec<f32> = Vec::new();
    let expected_period: usize = (sr / pitch) as usize;
    let sliding_size: usize = (expected_period as f64 * 0.1) as usize;
    let peak_sample = find_peak_sample(
        &unit.audio_file, 
        settings.start_sample, 
        settings.start_sample + expected_period * 2, 
        true
    );
    let corr_start = find_previous_zero(&unit.audio_file, peak_sample);
    

    let sample_list: SampleValueList = SampleValueList { index_list, value_list };
    return sample_list;
}