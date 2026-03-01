use std::{sync::Arc};
use std::collections::HashMap;
use realfft::{RealFftPlanner, RealToComplex};
use rustfft::num_complex::Complex;
use rustfft::{FftPlanner, Fft};
use crate::structures::BinFrame;
use crate::utils::{apply_hanning, get_bin_amp};

pub struct FftContext{
    real_planner: RealFftPlanner<f32>,
    complex_planner: FftPlanner<f32>,

    r2c_cache: HashMap<usize, Arc<dyn RealToComplex<f32>>>,
    c2c_cache: HashMap<usize, Arc<dyn Fft<f32>>>
}

impl FftContext{
    pub fn new() -> Self{
        Self { 
            real_planner: RealFftPlanner::new(), 
            complex_planner: FftPlanner::new(), 
            r2c_cache: HashMap::new(), 
            c2c_cache: HashMap::new(),
        }
    }

    pub fn get_r2c_plan(&mut self, nfft: usize) -> Arc<dyn RealToComplex<f32>>{
        self.r2c_cache.entry(nfft)
        .or_insert_with(|| self.real_planner.plan_fft_forward(nfft))
        .clone()
    }

    pub fn get_c2c_plan(&mut self, nfft: usize) -> Arc<dyn Fft<f32>>{
        self.c2c_cache.entry(nfft)
        .or_insert_with(|| self.complex_planner.plan_fft_inverse(nfft))
        .clone()
    }
}

pub fn process_r2c(input: &mut [f32], output: &mut [Complex<f32>], plan: &Arc<dyn RealToComplex<f32>>){
    if input.len() != plan.len() {panic!("Input size mismatch");}

    apply_hanning(input);

    plan.process(input, output).unwrap();
}

pub fn process_c2c(input: &mut [Complex<f32>], plan: &Arc<dyn Fft<f32>>){
    if input.len() != plan.len() {panic!("Buffer size mismatch");}

    plan.process(input);
}

pub fn get_complex_spectrum(input: &[f32], nfft: usize, context: &mut FftContext) -> Vec<Complex<f32>>{
    let plan = context.get_r2c_plan(nfft);
    let mut temp_input = input[0..nfft].to_vec();
    let mut output = plan.make_output_vec();

    process_r2c(&mut temp_input, &mut output, &plan);

    output
}

pub fn spectrum_to_bins(spectrum: &Vec<Complex<f32>>, n: usize, sr: f32) -> Vec<BinFrame>
{
    let mut spectral_bins: Vec<BinFrame> = Vec::with_capacity(spectrum.len()); // Optimization: pre-allocate memory
        let bin_width = sr / n as f32;

    for (i, complex_val) in spectrum.iter().enumerate() {
        
        let freq = i as f32 * bin_width;
        let phase: f32 = complex_val.arg(); 
        let amp: f32 = get_bin_amp(*complex_val, n);

        let bin_frame = BinFrame { freq, phase, amp };
        spectral_bins.push(bin_frame);
    }

    return spectral_bins;
}

pub fn interp_delta(spectral_bins: &Vec<BinFrame>, index: usize) -> f64
{
    if index <= 0 || index > spectral_bins.len() {return 0.0;}
    let m1: f64 = spectral_bins[index].amp as f64;
    let m0: f64 = spectral_bins[index - 1].amp as f64;
    let m2: f64 = spectral_bins[index + 1].amp as f64;
    let denom: f64 = m1 - 2.0 * m0 + m2;
    if denom.abs() < 1.0e-20 {return 0.0;}
    else {return 0.5 * (m1 - m2) / denom;}
}