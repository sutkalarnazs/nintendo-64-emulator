use glfw::{Action, Key};

pub struct InputHandler {
    keys: [bool; 256],
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler { keys: [false; 256] }
    }

    pub fn key_callback(&mut self, key: Key, action: Action) {
        if key as usize >= self.keys.len() {
            return;
        }
        self.keys[key as usize] = action == Action::Press;
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        if key as usize >= self.keys.len() {
            return false;
        }
        self.keys[key as usize]
    }
}