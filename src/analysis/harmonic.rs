use std::f32::consts::PI;
use realfft::num_complex::Complex;

use crate::models::{AudioInfo, HarmonicSettings};
use crate::dsp::stft::{FftContext, process_r2c};
use crate::results::HarmonicResults;
use crate::structures::{BinFrame, BinFreq};
use crate::utils::{apply_hanning, bin_to_hz, cmplx_to_amp, find_peak_index_vector, hz_to_bin, hz_to_midi, mag_to_amp, midi_to_hz};

pub fn process(unit: &AudioInfo, settings: &HarmonicSettings, context: &mut FftContext, top: &Vec<BinFrame>, start_sample: usize) -> HarmonicResults
{
    let analysis_len: usize;
    if start_sample > 0 {analysis_len = unit.audio_file.len() - start_sample;}
    else {analysis_len = unit.audio_file.len();}
    let num_frames = analysis_len / settings.hop_size - 1;
    let mut input_buffer: Vec<f32> = vec![0.0; settings.nfft];
    let plan = context.get_r2c_plan(settings.nfft);
    let mut output_buffer = plan.make_output_vec();
    let mut h_results: HarmonicResults = HarmonicResults { h_ratio_envs: vec![Vec::with_capacity(num_frames); settings.num_harmonics], index_list: Vec::new(), rms: 0.0 };

    for i in 0..(num_frames - 1){
        let i_samp = i * settings.hop_size + start_sample;
        if i_samp >= unit.audio_file.len() {break;}

        if i_samp + settings.nfft < unit.audio_file.len(){input_buffer = unit.audio_file[i_samp..i_samp + settings.nfft].to_vec();}
        else {
            for o in 0..settings.nfft
            {
                if i_samp + o < unit.audio_file.len() {
                    input_buffer[o] = unit.audio_file[i_samp + o];
                }
                else{
                    input_buffer[o] = 0.0;
                }
            }
        }

        if settings.apply_hanning{apply_hanning(&mut input_buffer);}

        process_r2c(&mut input_buffer, &mut output_buffer, &plan);

        for h in 1..top.len(){
            if h >= settings.num_harmonics {break;}
            if top[h - 1].freq <= 20.0 {continue;}

            let target_freq = top[h - 1].freq;
            let bin_freq = find_peak_within_tolerance(target_freq, settings.tolerance, settings.nfft, unit.sample_rate, &output_buffer);

            if bin_freq.bin < settings.nfft / 2 + 1 {
                let mut f_unit: FreqUnit = find_peak(bin_freq, &output_buffer, settings.nfft, unit.sample_rate, settings.bin_range as i32);
                if f_unit.amp.is_nan() {break;}
                if f_unit.amp > 1.0 {f_unit.amp = 1.0;}
                let frame: BinFrame = BinFrame { freq: f_unit.bin.freq, phase: f_unit.pha, amp: f_unit.amp };
                h_results.h_ratio_envs[h - 1].push(frame);
                h_results.index_list.push(i_samp);
            }

        }
    }

    if h_results.index_list.len() > h_results.h_ratio_envs[0].len()
    {
        h_results.index_list.resize(h_results.h_ratio_envs[0].len(), unit.audio_file.len() - 1);
    }
    else if h_results.index_list.len() < h_results.h_ratio_envs[0].len()
    {
        panic!("This shouldn't happen...");
    }

    let mut sum = 0.0;
    for i in start_sample..unit.sample_rate as usize{
        sum += unit.audio_file[i] * unit.audio_file[i];
    }
    h_results.rms = (sum / unit.sample_rate).sqrt();
    return h_results;
}

#[derive(Clone, Copy, Default, Debug)]
struct FreqUnit{
    pub bin: BinFreq,
    pub mag: f32,
    pub amp: f32,
    pub pha: f32
}

fn find_peak_within_tolerance(target_freq: f32, tolerance: f32, nfft: usize, sr: f32, out: &[Complex<f32>]) -> BinFreq{
    let midi_tolerance = tolerance / 100.0;

    let low_midi: f32 = hz_to_midi(target_freq) - midi_tolerance;
    let low_freq: f32 = midi_to_hz(low_midi);
    let low_bin: usize = hz_to_bin(low_freq, nfft, sr);

    let hi_midi: f32 = hz_to_midi(target_freq) + midi_tolerance;
    let hi_freq: f32 = midi_to_hz(hi_midi);
    let hi_bin: usize = hz_to_bin(hi_freq, nfft, sr);

    let target_bin: usize = hz_to_bin(target_freq, nfft, sr);

    let bin_number: usize = hi_bin.abs_diff(low_bin);

    let mut peak_amp: f32 = 0.0;
    let mut out_bin: BinFreq = BinFreq::default();

    if bin_number != 0{
        let mut no_amp: bool = true;

        for i in 0..bin_number{
            let current_bin = low_bin + i;
            let re = out[current_bin].re;
            let im = out[current_bin].im;

            let amp = cmplx_to_amp(im, re, nfft);

            if amp > peak_amp{
                no_amp = false;
                out_bin.bin = current_bin;
                out_bin.freq = bin_to_hz(current_bin, nfft, sr);
                peak_amp = amp;
            }
        }
        if no_amp{
            out_bin.bin = target_bin;
            out_bin.freq = bin_to_hz(target_bin, nfft, sr);
        }
    }
    else{
        out_bin.freq = target_freq;
        out_bin.bin = hz_to_bin(target_freq, nfft, sr);
    }

    return out_bin;

}

fn find_peak(in_target: BinFreq, out: &Vec<Complex<f32>>, nfft: usize, sr: f32, bin_range: i32) -> FreqUnit{

    let mut target = in_target;

    let mut log_km1 = out[target.bin - 1].re.hypot(
        out[target.bin - 1].re).ln();
    let mut log_k = out[target.bin].re.hypot(
        out[target.bin].im).ln();
    let mut log_kp1 = out[target.bin + 1].re.hypot(
        out[target.bin + 1].im).ln();
    
    let i_start: i32 = - (bin_range.abs() as i32);
    let i_end: i32 = bin_range.abs() + 1;
    if log_km1 > log_k || log_kp1 > log_k{
        let mut in_vec: Vec<f32> = Vec::new();
        for i in i_start..i_end{
            if target.bin as i32 + i > 0 && target.bin as i32 + 1 < out.len() as i32{
                let pos: usize = (target.bin as i32 + i) as usize;
                in_vec.push(out[pos].re.hypot(out[pos].im).ln());
            }
        }
        let diff: i32 = find_peak_index_vector(&in_vec) as i32 + i_start;
        if diff >= 0 {target.bin += diff as usize;}

        log_km1 = out[target.bin - 1].re.hypot(
            out[target.bin - 1].re).ln();
        log_k = out[target.bin].re.hypot(
            out[target.bin].im).ln();
        log_kp1 = out[target.bin + 1].re.hypot(
            out[target.bin + 1].im).ln();
    }

    let mut delta: f32 = 0.5 * (log_km1 - log_kp1) / (log_km1 - 2.0 * log_k + log_kp1);
    if delta > 1.0 || delta < 1.0{
        delta = 0.0;
    }

    let mut out_data = FreqUnit::default();
    let mut out_bin: BinFreq = BinFreq::default();
    out_bin.bin_f = target.bin as f32 + delta;
    out_bin.freq = bin_to_hz(out_bin.bin_f as usize, nfft, sr);

    out_data.bin = out_bin;

    let interp_log_mag = log_k - 0.25 * (log_km1 - log_kp1) * delta;
    out_data.mag = interp_log_mag.exp();
    out_data.amp = mag_to_amp(out_data.mag, nfft);

    let phase_at_bin = out[target.bin].im.atan2(out[target.bin].re);
    let phase = phase_at_bin - (PI * out_data.bin.bin_f);

    out_data.pha = (phase + PI) % (PI * 2.0) - PI;

    return out_data;
}