use realfft::{RealFftPlanner};
use crate::models;
use crate::results::OvertoneResults;
use crate::structures::BinFrame;

pub fn process(unit: &models::AudioInfo, settings: &models::OvertoneSettings) -> OvertoneResults
{
    //Value initialization
    let len = settings.fft_size;
    let mut results: OvertoneResults = Default::default();

    //FFT setup
    let mut planner = RealFftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(len);

    //Handle data from audio
    let buf_slice = &unit.audio_file[0..len];
    let mut window: Vec<f32> = Vec::new();
    window.copy_from_slice(buf_slice);

    //Create and initialize vectors
    let mut indata = window;
    let mut spectrum: Vec<realfft::num_complex::Complex<f32>> = fft.make_output_vec();

    //Error checking
    assert_eq!(indata.len(), len);
    assert_eq!(spectrum.len(), len/2 + 1);

    fft.process(&mut indata, &mut spectrum).unwrap();

    let mut freqs: Vec<f32> = Vec::new();
    let mut phases: Vec<f32> = Vec::new();

    for i in 0..len{
        freqs.push(spectrum[i].re);
        phases.push(spectrum[i].im);
    }

    return results;

}

fn find_relevant_overtones(spectrum: &Vec<realfft::num_complex::Complex<f32>>) -> Vec<BinFrame>
{
    let mut results: Vec<BinFrame> = Vec::new();

    return results;
}