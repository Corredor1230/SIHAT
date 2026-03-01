use realfft::num_complex::Complex;

use crate::structures::BinFrame;

//Hanning
pub fn hann_gain_rms() -> f32
{
    return 0.5;
}

pub fn apply_hanning(input: &mut [f32])
{
    //Recasting necessary values as data types
    let m_double = (input.len() - 1) as f64;
    
    for (i, sample) in input.iter_mut().enumerate() {
        let angle = 2.0 * std::f64::consts::PI * (i as f64) / m_double;
        let window_val = 0.5 * (1.0 - angle.cos());
        
        *sample = (*sample as f64 * window_val) as f32;
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

pub fn hz_to_bin(freq: f32, n: usize, sr: f32) -> usize
{
    let div: f32 = freq * n as f32 / sr;
    let bin: usize = div.round() as usize;
    return bin;
}

pub fn bin_to_hz(bin: usize, n: usize, sr:f32) -> f32
{
    if n == 0 {return 0.0};
    let freq: f32 = sr * bin as f32 / n as f32;
    return freq;
}

pub fn cents_to_hz(center_freq: f32, cents: f32) -> f32
{
    let exp = cents / 1200.0;
    let r = 2.0_f32.powf(exp);
    let cents = center_freq * (r - 1.0);
    let higher = cents.max(1.0e-11);
    return higher;
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

pub fn cmplx_to_amp(im: f32, re: f32, nfft: usize) -> f32
{
    let mag = (im * im + re * re).sqrt();
    return mag_to_amp(mag, nfft);
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

pub fn normalize_audio(audio: &mut Vec<f32>, db: f32)
{
    let target_linear = 10f32.powf(db / 20.0);
    let current_peak: f32 = audio.iter().fold(0.0f32, |max, &sample| max.max(sample.abs()));
    if current_peak > 0.0 {
        let scale_factor = target_linear / current_peak;
        for sample in audio.iter_mut() {
            *sample *= scale_factor;
        }
    }
}

//Math
pub fn is_within_tolerance(freq1: f32, freq2: f32, tolerance: f32) -> bool
{
    let mut result = false;
    let substr = freq1 - freq2;
    if substr.abs() < tolerance {result = true;}
    return result;
}

pub fn next_power_of_usize(val: usize, base: usize) -> usize
{
    if val < 0{
        panic!("Non-positive size");
    }
    let mut next_power = base;
    for i in 1..32{
        let prev_exp: f32 = i as f32 - 1.0;
        let prev_pow: usize = (base as f32).powf(prev_exp).round() as usize;
        let curr_exp: f32 = i as f32;
        let curr_pow: usize = (base as f32).powf(curr_exp).round() as usize;

        if val > prev_pow && val < curr_pow {next_power = curr_pow;}
    }

    return next_power;
}

pub fn prev_power_of_usize(val: usize, base: usize) -> usize
{
    if val < 0  {panic!("Non-positive usize");}
    let mut prev_power: usize = base;
    for i in 1..32{
        let prev_exp: f32 = i as f32 - 1.0;
        let prev_pow: usize = (base as f32).powf(prev_exp).round() as usize;
        let curr_exp: f32 = i as f32;
        let curr_pow: usize = (base as f32).powf(curr_exp).round() as usize;

        if val > prev_pow && val < curr_pow {prev_power = prev_pow;}
    }

    return prev_power;
}

pub fn closest_power_of_usize(val: usize, base: usize) -> usize
{
    if val < 0  {panic!("Non-positive usize");}
    let mut prev_power: usize = base;
    let mut next_power: usize = base;
    for i in 1..32{
        let prev_exp: f32 = i as f32 - 1.0;
        let prev_pow: usize = (base as f32).powf(prev_exp).round() as usize;
        let curr_exp: f32 = i as f32;
        let curr_pow: usize = (base as f32).powf(curr_exp).round() as usize;

        if val > prev_pow && val < curr_pow {
            prev_power = prev_pow;
            next_power = curr_pow;
        }
    }

    let mut closest: usize = prev_power;

    if val.abs_diff(next_power) < val.abs_diff(prev_power) {
        closest = next_power;
    }

    return closest;
}

//Finders
pub fn find_peak_sample(audio: &Vec<f32>, start_sample: usize, end_sample: usize, use_abs: bool) -> usize
{
    let mut peak_samp = start_sample;
    let mut peak_val: f32;
    if use_abs {peak_val = 0.0;}
    else {peak_val = -1.0;}
    let end;
    let start;

    //Safety checks
    if end_sample < audio.len() {end = end_sample;}
    else {end = audio.len();}
    if start_sample > 0 {start = start_sample;}
    else {start = 0;}

    for samp in start..end
    {
        if use_abs
        {
            if audio[samp].abs() > peak_val
            {
                peak_val = audio[samp].abs();
                peak_samp = samp;
            }
        }
        else {
            if audio[samp] > peak_val{
                peak_val = audio[samp];
                peak_samp = samp;
            }
        }
    }  
    return peak_samp;
}

pub fn find_peak_index_vector(input: &[f32]) -> usize
{
    let mut max_val: f32 = 0.0;
    let mut out_index: usize = 0;
    for i in 0..input.len(){
        if input[i] > max_val{
            max_val = input[i];
            out_index = i;
        }
        else {continue;}
    }

    return out_index;
}

pub fn find_previous_zero(audio: &Vec<f32>, start_sample: usize) -> usize
{
    let mut zero_sample: usize = 0;
    let start;
    if start_sample > 2 {start = start_sample;}
    else {start = 2;}
    for i in start..audio.len()
    {
        let zero_found: bool = (audio[i] >= 0.0 && audio[i - 2] <= 0.0) || (audio[i] <= 0.0 && audio[i] >= 0.0);

        if zero_found{
            if audio[i - 2].abs() <= audio[i - 1].abs() {zero_sample = i - 2;}
            else {zero_sample = i - 1;}
            break;
        }
    }
    return zero_sample;
}

pub fn find_next_zero(audio: &Vec<f32>, start_sample: usize) -> usize
{
    let mut zero_sample: usize = 0;
    for i in start_sample..audio.len()
    {
        if i <= 1 {break;}
        let zero_found = (audio[i] >= 0.0 && audio[i - 2] < 0.0) || (audio[i] <= 0.0 && audio[i - 2] > 0.0);

        if zero_found {
            if audio[i - 2].abs() <= audio[i - 1].abs() {zero_sample = i - 2;}
            else {zero_sample = i - 1;}
            break;
        }
    }

    return zero_sample;
}

pub fn find_nearest_zero(audio: &Vec<f32>, start_sample: usize) -> usize
{
    let prev_zero = find_previous_zero(audio, start_sample);
    let next_zero = find_next_zero(audio, start_sample);
    let distance_prev = start_sample.abs_diff(prev_zero);
    let distance_next = start_sample.abs_diff(next_zero);

    if distance_prev < distance_next{return prev_zero;}
    else {return  next_zero;}
}