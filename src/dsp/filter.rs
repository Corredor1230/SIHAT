use crate::models::AudioInfo;

pub fn filtfilt(unit: &AudioInfo, cutoff_freq: f32)
{
    assert!(unit.audio_file.is_empty());
    assert!(unit.audio_file.len() < 2);

    let inner_u = std::f64::consts::PI * cutoff_freq as f64 / unit.sample_rate as f64;
    
}