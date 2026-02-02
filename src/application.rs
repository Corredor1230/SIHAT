use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use rodio::{Decoder, Source};
use crate::{analysis::analyzer, models::{AudioInfo, SihatSettings}, utils::normalize_audio};

pub fn run()
{
    let filename: String = "AGuit1_329_F.wav".to_string();
    let path = Path::new("src").join(filename.clone());
    let file = File::open(path).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();
    let channels = source.channels();
    let sample_rate = source.sample_rate();
    let mut audio: Vec<f32> = source.collect();
    let norm_db: f32 = -1.0;
    normalize_audio(&mut audio, norm_db);

    let audio_info: AudioInfo = AudioInfo { audio_file: audio, sample_rate: sample_rate as f32, file_name: filename };

    println!("Channels: {}, Sample Rate: {}", channels, sample_rate);

    let settings: SihatSettings = Default::default();
    analyzer::analyze(&audio_info, &settings);
}