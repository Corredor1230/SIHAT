use crate::structures;

#[derive(Default, Debug)]
pub struct TransientResults
{
    pub range: structures::SampleRange,
    pub t_length: usize,
    pub scalogram: Vec<structures::VariableRatePartial>,
    pub scalo_range: structures::SampleRange,
    pub scalogram_length: usize,
    pub rms: f32
}

#[derive(Default, Debug)]
pub struct HarmonicResults
{
    pub h_ratio_envs: Vec<Vec<structures::BinFrame>>,
    pub index_list: Vec<usize>,
    pub rms: f32
}

#[derive(Default, Debug)]
pub struct SihatResults
{
    pub o_results: Vec<structures::BinFrame>,
    pub t_results: TransientResults,
    pub h_results: HarmonicResults
}