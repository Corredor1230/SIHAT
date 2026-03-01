use crate::dsp::stft::{FftContext, get_complex_spectrum, spectrum_to_bins, interp_delta};
use crate::models;
use crate::models::OvertoneSettings;
use crate::structures::BinFrame;
use crate::utils::cents_to_hz;
use crate::utils::db_to_amp;
use crate::utils::is_within_tolerance;

pub fn process(unit: &models::AudioInfo, settings: &models::OvertoneSettings, context: &mut FftContext) -> Vec<BinFrame>
{
    //Value initialization
    let len = settings.fft_size;
    let init: usize = settings.init_sample;
    let sr: f32 = unit.sample_rate;

    let audio_data = &unit.audio_file[init..(init + len)];

    //let mut context: FftContext = FftContext::new();

    //Decompose spectrogram into BinFrames

    let complex_data = get_complex_spectrum(&audio_data, len, context);

    let spectral_bins: Vec<BinFrame> = spectrum_to_bins(&complex_data, len, sr);

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

    results.sort_by(|a, b| a.freq.partial_cmp(&b.freq).unwrap_or(std::cmp::Ordering::Equal));
    return results;
}