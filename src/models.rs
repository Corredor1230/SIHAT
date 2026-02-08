pub struct CorrelationSettings
{
    pub window_size: usize, 
    pub start_sample: usize, 
    pub stop_sample: usize, 
    pub go_left: bool, 
    pub jump_post_peak: bool, 
    pub jump_size: usize, 
    pub use_filter: bool, 
    pub is_low_pass: bool, 
    pub threshold: f32,
    pub cutoff: f32
}

impl Default for CorrelationSettings
{
    fn default() -> Self {
        CorrelationSettings { 
            window_size: 200, 
            start_sample: 10000, 
            stop_sample: 4000, 
            go_left: true, 
            jump_post_peak: true, 
            jump_size: 160, 
            use_filter: true, 
            is_low_pass: true, 
            threshold: 0.95,
            cutoff: 2000.0 
        }
    }
}


pub struct OvertoneSettings
{
    pub fft_size: usize,
    pub n_values: usize,
    pub threshold: f32,
    pub tolerance_in_cents: f32,
    pub init_sample: usize
}

impl Default for OvertoneSettings
{
    fn default() -> Self {
        OvertoneSettings { 
            fft_size: 32768, 
            n_values: 32,
            threshold: -60.0,
            tolerance_in_cents: 100.0,
            init_sample: 8000
        }
    }
}

pub struct TransientSettings
{
    pub rms_sample_size: usize,
    pub use_ms_size: bool,
    pub rms_ms_size: f32,
    pub set_hop_size: bool,
    pub hop_size: usize,
    pub start_sample: usize,
    pub rms_factor: f64,
    pub rms_threshold: f64,
    pub correlation_offset: usize,
    pub correlation_threshold: f32,
    pub pitch_multiplier: f32
}

impl Default for TransientSettings
{
    fn default() -> Self {
        TransientSettings { 
            rms_sample_size: 128,
            use_ms_size: false,
            rms_ms_size: 20.0,
            set_hop_size: false,
            hop_size: 128,
            start_sample: 2000,
            rms_factor: 3.0,
            rms_threshold: 0.1,
            correlation_offset: 1000,
            correlation_threshold: 0.85,
            pitch_multiplier: 8.0
        }
    }
}

pub struct SihatSettings
{
    pub o_settings: OvertoneSettings,
    pub t_settings: TransientSettings
}

impl Default for SihatSettings
{
    fn default() -> Self {
        SihatSettings { 
            o_settings: Default::default(),
            t_settings: Default::default()
        }
    }
}

pub struct AudioInfo
{
    pub audio_file: Vec<f32>,
    pub sample_rate: f32,
    pub file_name: String,
    pub meta_pitch: f32
}

impl Default for AudioInfo
{
    fn default() -> Self {
        AudioInfo { 
            audio_file: Vec::new(), 
            sample_rate: 44100.0, 
            file_name: "file.wav".to_string(),
            meta_pitch: 0.0 }
    }
}