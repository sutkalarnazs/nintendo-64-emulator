use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;

pub struct AudioManager {
    stream: OutputStream,
    sink: Sink,
}

impl AudioManager {
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        AudioManager { stream, sink }
    }

    pub fn play_sound(&self, file_path: &str) {
        let file = File::open(file_path).unwrap();
        let source = Decoder::new(file).unwrap();
        self.sink.append(source);
    }
}