// TODO:
    // Figure out how to play/pause music



use std::fs::File;
use std::io::BufReader;
use rodio::Source;

pub struct MusicPlayer {
    // device: rodio::,

}


fn play_music() {
    // Here's an example of how to run a sound effect
    let device = rodio::default_output_device().unwrap();
    let file = File::open("src/data/redial.mp3").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    rodio::play_raw(&device, source.convert_samples());
}