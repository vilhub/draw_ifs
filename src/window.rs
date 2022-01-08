use minifb::{ScaleMode, Window, WindowOptions, Key};

use crate::{state};

pub fn draw_window(width: usize, height: usize, update_state: &dyn Fn(&mut state::State), state: &mut state::State) {

    let mut window = Window::new(
        "Press ESC to exit",
        width,
        height,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::UpperLeft,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create window");

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    
    let mut size = (0, 0);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let new_size = (window.get_size().0, window.get_size().1);
        if new_size != size {
            size = new_size;
            state.buffer.resize(size.0 * size.1, 0);
        }
        update_state(state);

        window
            .update_with_buffer(&state.buffer, new_size.0, new_size.1)
            .unwrap();
    }
}