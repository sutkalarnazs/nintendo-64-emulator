use glfw::Window;

pub fn render(window: &Window, emulator: &super::emulator::Emulator) {
    let frame_buffer = emulator.get_frame_buffer();
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::DrawPixels(640, 480, gl::RGBA, gl::UNSIGNED_BYTE, frame_buffer.as_ptr() as *const _);
    }
}