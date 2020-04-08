extern crate rodio;

use std::io::BufReader;

pub fn play_prelude() {
    let device = rodio::default_output_device().unwrap();
    let sink = rodio::Sink::new(&device);
    let file = std::fs::File::open("assets/music/prelude.mp3").unwrap();

    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
}
