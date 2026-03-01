use core::f32;
use std::f32::consts::PI;
use std::cmp::Ordering;

use crate::{dsp::stft::{FftContext, process_c2c, process_r2c}, models::{AudioInfo, WaveletSettings}, structures::VariableRatePartial, utils::mag_to_amp};
use rustfft::{num_complex::Complex};

pub fn analyze_wavelet(signal: &[f32], s: WaveletSettings, nfft: usize, unit: &AudioInfo, context: &mut FftContext) -> Vec<VariableRatePartial>
{
    let mut f_dom_buffer: Vec<Complex<f32>> = vec![Complex::default(); nfft / 2 + 1];
    let mut inv_buffer: Vec<Complex<f32>> = vec![Complex::default(); nfft];
    let mut in_buffer: Vec<f32>  = vec![0.0; nfft];
    let fwd_plan = context.get_r2c_plan(nfft);

    let best_freqs: Vec<f32> = get_peaks(signal, unit.sample_rate, nfft, &s, context, &mut f_dom_buffer);

    let mut scalogram: Vec<VariableRatePartial> = Vec::with_capacity(best_freqs.len());

    in_buffer.copy_from_slice(signal);
    process_r2c(&mut in_buffer, &mut f_dom_buffer, &fwd_plan);

    for freq in best_freqs{
        let mut hop: usize = 1;

        if freq < 100.0         {hop = 256;}
        else if freq < 500.0    {hop = 64;}
        else if freq < 2000.0   {hop = 16;}
        else if freq < 8000.0   {hop = 4;}

        let full_res_envelope: Vec<f32> = extract_envelope(freq, nfft, unit.sample_rate, &mut f_dom_buffer, &mut inv_buffer, context);

        let partial: VariableRatePartial = VariableRatePartial { freq, hop_size: hop, data: full_res_envelope };

        scalogram.push(partial);
    }

    return scalogram;

}

fn get_peaks(signal: &[f32], sr: f32, nfft: usize, s: &WaveletSettings, context: &mut FftContext, f_dom_buffer: &mut Vec<Complex<f32>>) -> Vec<f32>
{
    let mut scan_freqs: Vec<f32> = Vec::new();
    let log_min = s.min_freq.log10();
    let log_max = s.max_freq.log10();

    let fwd_plan = context.get_r2c_plan(nfft);

    for i in 0..s.scan_res{
        let t = i as f32 / (s.scan_res - 1) as f32;
        let exp = log_min + (log_max - log_min) * t;
        let f = 10.0_f32.powf(exp);
        scan_freqs.push(f);
    }

    let mut energy_profile: Vec<f32> = Vec::new();
    let mut in_buffer = signal[0..nfft].to_vec();
    process_r2c(&mut in_buffer,  f_dom_buffer, &fwd_plan);

    for i in 0..s.scan_res{
        let w_filter = wavelet(scan_freqs[i], nfft, sr);
        let mut band_energy = 0.0;
        for k in 0..(nfft / 2 + 1){
            let rs = f_dom_buffer[k].re;
            let is = f_dom_buffer[k].im;
            let w = w_filter[k];

            let mag_sq = w * w * (rs * rs + is * is);
            band_energy += mag_sq;
        }

        energy_profile.push(band_energy);
    }

    let mut winners: Vec<f32> = Vec::new();
    let suppression_width: usize = 50;

    for _k in 0..s.num_partials{
        let max_info = energy_profile
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        if let Some((idx, &max_val)) = max_info {
            // Threshold check (using 1e-5 for clarity)
            if max_val < 0.00001 {
                break;
            }

            // Record the winner
            winners.push(scan_freqs[idx]);

            // Nuke neighbors
            let start = idx.saturating_sub(suppression_width);
            let end = (idx + suppression_width).min(energy_profile.len());

            for val in &mut energy_profile[start..end] {
                *val = 0.0;
            }
        } 
        else 
        {
            break; // energy_profile was empty
        }
    }

    winners.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    return winners;
    
}

fn extract_envelope(freq: f32, nfft: usize, sr: f32, f_dom_buffer: &mut Vec<Complex<f32>>, inv_buffer: &mut Vec<Complex<f32>>, context: &mut FftContext) -> Vec<f32>
{
    let wave = wavelet(freq, nfft, sr);
    let mut envelope: Vec<f32> = Vec::with_capacity(nfft);

    for k in 0..nfft{
        if k < (nfft / 2 + 1){
            let w: f32 = wave[k];
            inv_buffer[k].re = f_dom_buffer[k].re * w;
            inv_buffer[k].im = f_dom_buffer[k].im * w;
        }
        else{
            inv_buffer[k].re = 0.0;
            inv_buffer[k].im = 0.0;
        }
    }
    
    let inv_plan = context.get_c2c_plan(nfft);
    process_c2c(inv_buffer, &inv_plan);

    for i in 0..nfft{
        let re = inv_buffer[i].re;
        let im = inv_buffer[i].im;

        let inner = re * re + im * im;
        envelope.push(mag_to_amp(inner.sqrt(), nfft));
    }

    return envelope;

}

fn wavelet(freq: f32, nfft: usize, sr: f32) -> Vec<f32> {
    let mut wave: Vec<f32> = Vec::with_capacity(nfft / 2 + 1);
    let omega0: f32 = 6.0;
    let fc = omega0 / (PI * 2.0);
    let scale = (fc * sr) / freq;

    for i in 0..(nfft / 2 + 1){
        let omegak = (PI * 2.0 * i as f32) / nfft as f32;
        let exponent = scale * omegak - omega0;
        let newexp = -0.5 * exponent * exponent;
        let weight = 2.0 * newexp.exp();
        wave.push(weight);
    }
    return wave;
}