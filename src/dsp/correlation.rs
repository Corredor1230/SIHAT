use crate::structures::SampleValueList;

pub fn perform_cross_correlation(audio: &Vec<f32>, window_size: usize, start_sample: usize, stop_sample: usize, go_left: bool, jump_post_peak: bool, jump_size: usize) -> SampleValueList
{
    let mut index_list: Vec<usize> = Vec::new();
    let mut value_list: Vec<f32> = Vec::new();



    let sample_list: SampleValueList = SampleValueList { index_list, value_list };
    return sample_list;
}