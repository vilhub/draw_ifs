use minifb::{ScaleMode, Window, WindowOptions, Key};

use crate::{state};

pub struct Painter {
    pub state: state::State,
    pub width: usize,
    pub height: usize
}

pub trait Paintable {
    fn update_state(state: &mut state::State);
    fn handle_key_presses(keys: Vec<Key>, state: &mut state::State);
    fn handle_key_releases(keys: Vec<Key>, state: &mut state::State);
}

impl Painter{
    pub fn draw_window(&mut self) {

        let mut window = Window::new(
            "Press ESC to exit",
            self.width,
            self.height,
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
                self.state.buffer.resize(size.0 * size.1, 0);
            }
            Self::update_state(&mut self.state);
            Self::handle_key_presses(window.get_keys(), &mut self.state);
            Self::handle_key_releases(window.get_keys_released(), &mut self.state);

            window
                .update_with_buffer(&self.state.buffer, new_size.0, new_size.1)
                .unwrap();
        }
    }
}