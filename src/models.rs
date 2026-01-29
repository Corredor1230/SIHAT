pub struct OvertoneSettings
{
    pub fft_size: usize
}

impl Default for OvertoneSettings
{
    fn default() -> Self {
        OvertoneSettings { 
            fft_size: 2048 
        }
    }
}

pub struct SihatSettings
{
    pub o_settings: OvertoneSettings
}

impl Default for SihatSettings
{
    fn default() -> Self {
        SihatSettings { o_settings: Default::default() }
    }
}

pub struct AudioInfo
{
    pub audio_file: Vec<f32>,
    pub sample_rate: f32,
    pub file_name: String
}

impl Default for AudioInfo
{
    fn default() -> Self {
        AudioInfo { audio_file: Vec::new(), sample_rate: 44100.0, file_name: "file.wav".to_string() }
    }
}