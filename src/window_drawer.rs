use minifb::{ScaleMode, Window, WindowOptions, Key};

use crate::{ifs_computer::IFSComputer, WIDTH, HEIGHT, frame::Frame, Point2};

pub fn draw_window(ifs_computer: &mut IFSComputer) {

    let mut window = Window::new(
        "Press ESC to exit",
        WIDTH,
        HEIGHT,
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

    let mut frame = Frame{ buffer: vec![0; WIDTH * HEIGHT],
                                 size: Point2 { x: WIDTH as u32, y: HEIGHT as u32 } };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let new_size = (window.get_size().0, window.get_size().1);
        if new_size != size {
            size = new_size;
            frame.buffer.resize(size.0 * size.1, 0);
        }
        ifs_computer.update_state();
        ifs_computer.handle_key_presses(window.get_keys());
        ifs_computer.handle_key_releases(window.get_keys_released());
        ifs_computer.draw_on_frame(&mut frame);

        window
            .update_with_buffer(&frame.buffer, new_size.0, new_size.1)
            .unwrap();
    }
}