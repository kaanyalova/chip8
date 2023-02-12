use std::io::BufReader;

use rodio::{OutputStreamHandle, Sink, Source};

pub fn setup_beep(stream: OutputStreamHandle) -> Sink {
    let f = std::fs::File::open("assets/beep.wav").unwrap();
    let sink = rodio::Sink::try_new(&stream).unwrap();
    let src = rodio::Decoder::new(BufReader::new(f)).unwrap();
    sink.pause();
    sink.append(src.repeat_infinite());
    sink
}

pub fn start_beep(sink: &Sink) {
    sink.play();
}

pub fn stop_beep(sink: &Sink) {
    sink.pause();
}
