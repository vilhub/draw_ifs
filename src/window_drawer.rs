use std::{
    sync::{Arc, Mutex},
    thread,
};

use minifb::{Key, ScaleMode, Window, WindowOptions};

use crate::{
    constants::HEIGHT,
    constants::WIDTH,
    frame::Frame,
    ifs_computer::{draw_on_frame, handle_key_presses, handle_key_releases, IFSComputer},
    Point2,
};

pub fn draw_window(ifs_computer: IFSComputer) {
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

    // Limit to max ~60 fps update rate => 16600
    window.limit_update_rate(Some(std::time::Duration::from_micros(33200)));

    let mut draw_frame = Frame {
        buffer: vec![0; WIDTH * HEIGHT],
        size: Point2 {
            x: WIDTH as u32,
            y: HEIGHT as u32,
        },
    };
    let compute_frame = Frame {
        buffer: vec![0; WIDTH * HEIGHT],
        size: Point2 {
            x: WIDTH as u32,
            y: HEIGHT as u32,
        },
    };

    let mut size = (WIDTH, HEIGHT);
    let histogram = Mutex::new(compute_frame);

    let compute_histogram = Arc::new(histogram);
    let draw_histogram = compute_histogram.clone();
    let _handle = thread::spawn(move || {
        ifs_computer.compute_ifs(&compute_histogram);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let new_size = (window.get_size().0, window.get_size().1);
        if new_size != size {
            size = new_size;
            draw_frame.buffer.resize(size.0 * size.1, 0);
        }
        handle_key_presses(window.get_keys());
        handle_key_releases(window.get_keys_released());
        draw_on_frame(&mut draw_frame, &draw_histogram.lock().unwrap());

        window
            .update_with_buffer(&draw_frame.buffer, new_size.0, new_size.1)
            .unwrap();
    }
}
