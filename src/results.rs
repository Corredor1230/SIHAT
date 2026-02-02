use crate::structures;

#[derive(Default, Debug)]
pub struct TransientResults
{
    pub range: structures::SampleRange,
    pub scalogram: Vec<structures::VariableRatePartial>,
    pub rms: f32
}

#[derive(Default, Debug)]
pub struct SihatResults
{
    pub o_results: Vec<structures::BinFrame>,
    pub t_results: TransientResults
}