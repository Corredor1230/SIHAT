use realfft::{RealFftPlanner};
use crate::models;
use crate::models::OvertoneSettings;
use crate::structures::BinFrame;
use crate::utils::apply_hanning;
use crate::utils::cents_to_hz;
use crate::utils::db_to_amp;
use crate::utils::interp_delta;
use crate::utils::is_within_tolerance;
use crate::utils::spectrum_to_bins;

pub fn process(unit: &models::AudioInfo, settings: &models::OvertoneSettings) -> Vec<BinFrame>
{
    //Value initialization
    let len = settings.fft_size;
    let init: usize = settings.init_sample;
    let sr: f32 = unit.sample_rate;

    //FFT setup
    let mut planner = RealFftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(len);

    //Handle data from audio
    let mut indata = unit.audio_file[init..(len + init)].to_vec();
    let mut spectrum: Vec<realfft::num_complex::Complex<f32>> = fft.make_output_vec();

    //Error checking
    assert_eq!(indata.len(), len);
    assert_eq!(spectrum.len(), len/2 + 1);

    //Apply hann
    apply_hanning(&mut indata);

    fft.process(&mut indata, &mut spectrum).unwrap();

    //Decompose spectrogram into BinFrames
    let spectral_bins: Vec<BinFrame> = spectrum_to_bins(&spectrum, len, sr);

    let results = find_relevant_overtones(spectral_bins, sr as f64, &settings);

    return results;

}

fn find_relevant_overtones(s_bins: Vec<BinFrame>, sr: f64, settings: &OvertoneSettings) -> Vec<BinFrame>
{
    let n: f64 = settings.fft_size as f64;
    let thresh: f32 = db_to_amp(settings.threshold);
    let tolerance: f32 = settings.tolerance_in_cents;
    let num_bins: usize = s_bins.len();
    let mut candidates: Vec<BinFrame> = Vec::new();
    let mut results: Vec<BinFrame> = Vec::new();

    for i in 1..(num_bins - 1)
    {
        if s_bins[i].amp > s_bins[i - 1].amp &&
        s_bins[i].amp > s_bins[i + 1].amp
        {
            let delta: f64 = interp_delta(&s_bins, i);
            let f: f64 = (i as f64 + delta) * (sr / n);
            let bin: BinFrame = BinFrame { freq: f as f32, phase: s_bins[i].phase, amp: s_bins[i].amp };
            candidates.push(bin);
        }
    }
    candidates.sort_by(|a, b| b.amp.partial_cmp(&a.amp).unwrap_or(std::cmp::Ordering::Equal));

    let mut i: usize = 0;

    while i < candidates.len() && results.len() < settings.n_values {
        let seed = BinFrame { freq: candidates[i].freq, phase: candidates[i].phase, amp: candidates[i].amp };
        
        // Increment i for the NEXT loop, but use the current 'i' (0) now
        i += 1; 

        // Logic checks
        if seed.freq < 60.0 || seed.freq > 19999.0 || seed.amp < thresh {
            continue;
        }

        let tol_in_hz = cents_to_hz(seed.freq, tolerance);
        let mut within_tolerance = false;

        // Check against existing results
        for res in &results {
            if is_within_tolerance(res.freq, seed.freq, tol_in_hz) {
                within_tolerance = true;
                break; // Stop looking if we found a match
            }
        }

        if !within_tolerance {
            results.push(seed);
        }
    }

    candidates.sort_by(|a, b| b.freq.partial_cmp(&a.freq).unwrap_or(std::cmp::Ordering::Equal));
    return results;
}