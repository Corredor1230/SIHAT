//Hanning
pub fn hann_gain_rms() -> f32
{
    return 0.5;
}

pub fn apply_hanning(input: &mut Vec<f32>)
{
    
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
pub fn mag_to_amp(mag: f32, n: i32) -> f32
{
    let mut a: f32 = (2.0 * mag) / n as f32;
    a /= hann_gain_rms();
    return a;
}

pub fn amp_to_mag(amp: f32, n: i32) -> f32
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

