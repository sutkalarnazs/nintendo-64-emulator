use glfw::{Action, Context, Key};
use std::ffi::CString;
use std::ptr;
use std::sync::mpsc::Receiver;

mod graphics;
mod input;
mod audio;
mod emulator;

fn main() {
    let mut glfw = glfw::init(glfw::Action::Press).unwrap();
    let (mut window, events) = glfw.create_window(800, 600, "Nintendo 64 Emulator", glfw::WindowMode::Windowed).unwrap();
    window.make_current();
    window.set_key_polling(true);

    let mut emulator = emulator::Emulator::new();
    emulator.load_rom("path/to/rom.n64");

    while !window.should_close() {
        process_events(&mut window, &events);
        emulator.run_frame();
        graphics::render(&window, &emulator);
        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}