use std::fs::File;
use std::io::Read;

pub struct Emulator {
    frame_buffer: Vec<u8>,
}

impl Emulator {
    pub fn new() -> Self {
        Emulator { frame_buffer: vec![0; 640 * 480 * 4] }
    }

    pub fn load_rom(&mut self, path: &str) {
        let mut file = File::open(path).unwrap();
        let mut rom_data = Vec::new();
        file.read_to_end(&mut rom_data).unwrap();
        self.initialize(rom_data);
    }

    fn initialize(&mut self, rom_data: Vec<u8>) {
        // Initialize emulator state with ROM data
    }

    pub fn run_frame(&mut self) {
        // Run a single frame of the emulator
    }

    pub fn get_frame_buffer(&self) -> &Vec<u8> {
        &self.frame_buffer
    }
}