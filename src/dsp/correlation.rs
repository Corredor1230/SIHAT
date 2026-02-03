use crate::{models::{AudioInfo, CorrelationSettings}, structures::SampleValueList};

pub fn perform_cross_correlation(unit: &AudioInfo, pitch: f32, settings: &CorrelationSettings) -> SampleValueList
{
    //Initialize
    let sr = unit.sample_rate;
    let mut index_list: Vec<usize> = Vec::new();
    let mut value_list: Vec<f32> = Vec::new();
    let expected_period: usize = (sr / pitch) as usize;
    

    let sample_list: SampleValueList = SampleValueList { index_list, value_list };
    return sample_list;
}