use std::fs::File;
use std::io::BufReader;

use rodio::{Decoder, Source};

mod analysis;
mod application;
mod models;
mod utils;
mod results;
mod structures;

fn main(){
    let file = File::open("src/AGuit1_329_F.wav").unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();

    let channels = source.channels();
    let sample_rate = source.sample_rate();
    println!("Channels: {}, Sample Rate: {}", channels, sample_rate);

    application::run();
}