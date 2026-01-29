use realfft::num_complex::Complex;

use crate::structures::BinFrame;

//Hanning
pub fn hann_gain_rms() -> f32
{
    return 0.5;
}

pub fn apply_hanning(input: &mut Vec<f32>, n: usize)
{
    //Recasting necessary values as data types
    let n_double: f64 = n as f64;
    let m_double: f64 = n as f64 - 1.0;

    for i in 0..n
    {
        let angle: f64 = 2.0 * std::f64::consts::PI * n_double / m_double;
        let window_val = 0.5 * (1.0 - angle.cos());

        input[i] = (input[i] as f64 * window_val) as f32;
    }
}

//Frequency
pub fn midi_to_hz(midi: f32) -> f32
{
    let freq: f32 = 440.0 * 2.0_f32.powf((midi - 69.0) / 12.0);
    return freq;
}

pub fn hz_to_midi(freq: f32) -> f32
{
    let ratio: f32 = freq / 440.0;
    let midi: f32 = 12.0 * ratio.log2() + 69.0;
    return midi;
}

pub fn freq_to_bin(freq: f32, n: i32, sr: f32) -> i32
{
    let div: f32 = freq * n as f32 / sr;
    let bin: i32 = div.round() as i32;
    return bin;
}

pub fn bin_i32_to_freq(bin: i32, n: i32, sr:f32) -> f32
{
    if n == 0 {return 0.0};
    let freq: f32 = sr * bin as f32 / n as f32;
    return freq;
}

//Amplitude
pub fn mag_to_amp(mag: f32, n: usize) -> f32
{
    let mut a: f32 = (2.0 * mag) / n as f32;
    a /= hann_gain_rms();
    return a;
}

pub fn amp_to_mag(amp: f32, n: usize) -> f32
{
    let m: f32 = (amp * n as f32) / 4.0;
    return m;
}

pub fn db_to_amp(db: f32) -> f32
{
    let amp: f32 = 10.0_f32.powf(db / 20.0);
    return amp;
}

pub fn amp_to_db(amp: f32) -> f32
{
    let amp_div: f32 = amp.abs() / 1.0;
    if amp_div <= 0.0 {return -120.0;}
    let db: f32 = 20.0 * amp_div.log10();
    return db;
}

pub fn get_bin_mag(bin: Complex<f32>) -> f32
{
    let re: f32 = bin.re;
    let im: f32 = bin.im;
    let sq: f32 = re * re + im * im;
    let mag: f32 = sq.sqrt();
    return mag;
}

pub fn get_bin_amp(bin: Complex<f32>, n: usize) -> f32
{
    let mag: f32 = get_bin_mag(bin);
    let amp: f32 = mag_to_amp(mag, n);
    return amp;
}

//Math
pub fn spectrum_to_bins(spectrum: Vec<Complex<f32>>, n: usize) -> Vec<BinFrame>
{
    let mut spectral_bins: Vec<BinFrame> = Vec::new();
    for i in 0..n{
        let freq = spectrum[i].re;
        let phase: f32 = spectrum[i].im;
        let amp: f32 = get_bin_amp(spectrum[i], n);
        let bin_frame: BinFrame = BinFrame { freq, phase, amp };
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

