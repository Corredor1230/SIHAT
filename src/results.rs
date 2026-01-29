#[derive(Default, Debug)]
pub struct OvertoneResults
{
    pub freqs: Vec<f32>,
    pub amps: Vec<f32>,
    pub phases: Vec<f32>
}

#[derive(Default, Debug)]
pub struct SihatResults
{
    pub o_results: OvertoneResults
}