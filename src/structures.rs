#[derive(Clone, Copy, Default, Debug)]
pub struct BinFrame
{
    pub freq: f32,
    pub phase: f32,
    pub amp: f32
}

#[derive(Clone, Copy, Default, Debug)]
pub struct SampleRange
{
    pub init: usize,
    pub end: usize
}

#[derive(Clone, Default, Debug)]
pub struct VariableRatePartial
{
    pub freq: f32,
    pub hop_size: usize,
    pub data: Vec<f32>
}

#[derive(Clone, Default, Debug)]
pub struct SampleValueList
{
    pub index_list: Vec<usize>,
    pub value_list: Vec<f32>
}