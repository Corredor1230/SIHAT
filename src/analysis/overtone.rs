use realfft::{RealFftPlanner};
use crate::models;
use crate::models::OvertoneSettings;
use crate::structures::BinFrame;
use crate::utils::apply_hanning;
use crate::utils::db_to_amp;
use crate::utils::interp_delta;
use crate::utils::spectrum_to_bins;

pub fn process(unit: &models::AudioInfo, settings: &models::OvertoneSettings) -> Vec<BinFrame>
{
    //Value initialization
    let len = settings.fft_size;
    let mut results: Vec<BinFrame> = Vec::new();

    //FFT setup
    let mut planner = RealFftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(len);

    //Handle data from audio
    let mut indata = unit.audio_file[0..len].to_vec();
    let mut spectrum: Vec<realfft::num_complex::Complex<f32>> = fft.make_output_vec();

    //Error checking
    assert_eq!(indata.len(), len);
    assert_eq!(spectrum.len(), len/2 + 1);

    //Apply hann
    apply_hanning(&mut indata, len);

    fft.process(&mut indata, &mut spectrum).unwrap();

    //Decompose spectrogram into BinFrames
    let spectral_bins: Vec<BinFrame> = spectrum_to_bins(spectrum, len);

    results = find_relevant_overtones(spectral_bins, unit.sample_rate as f64, &settings);

    return results;

}

fn find_relevant_overtones(s_bins: Vec<BinFrame>, sr: f64, settings: &OvertoneSettings) -> Vec<BinFrame>
{
    let n: f64 = settings.fft_size as f64;
    let thresh: f32 = db_to_amp(settings.threshold);
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
    let mut processed_bins = vec![false; candidates.len()];

    loop {
        i += 1;
        if i > candidates.len() || results.len() > settings.n_values {break;}
        if processed_bins[i] {continue;}

        let seed = &candidates[i];



    }

    return results;
}