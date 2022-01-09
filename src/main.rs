use std::time::SystemTime;

use algebra::Point2;
use minifb::Key;
use rand::Rng;

mod painter;
mod state;
mod frame;
mod algebra;

const WIDTH: usize = 640;
const HEIGHT: usize = 640;

const DOMAIN: frame::Domain = frame::Domain { min: Point2{x: -1., y:-1.},
                                              max: Point2{x: 1., y: 1.} };

pub fn compute_ifs(frame: &mut frame::Frame) {
    let mut iter_point = Point2{ x: 0.1, y: 0.1 };
    let mut rng = rand::thread_rng();
    for i in 1..10000 {
        match rng.gen_range(1..=3) { // Optimize sampling
            1 => iter_point = (iter_point + Point2{ x: DOMAIN.min.x, y: DOMAIN.min.y }) / 2.,
            2 => iter_point = (iter_point + Point2{ x: DOMAIN.max.x, y: DOMAIN.min.y }) / 2.,
            3 => iter_point = (iter_point + Point2{ x: DOMAIN.max.x, y: DOMAIN.max.y }) / 2.,
            _ => ()
        }
        if i > 20 {
            let frame_point = frame.from_domain(iter_point, DOMAIN);
            frame.increment_pixel(frame_point);
        }
    }
}

impl painter::Paintable for painter::Painter {
    fn update_state(state: &mut state::State) {

        let _time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as u32 / 100;
        
        compute_ifs(&mut state.histogram);
        for (i, j) in state.frame.buffer.iter_mut().zip(state.histogram.buffer.iter()) {
            let intensity = std::cmp::min(*j, 0xFF);
            *i = intensity << 16 | intensity << 8 | intensity;
        }
    }
    
    fn handle_key_presses(keys: Vec<Key>, state: &mut state::State) {        
        keys.iter().for_each(|key| match key {
            Key::W => state.increment += 1,
            Key::T => println!("holding t!"),
            _ => (),
        });
    }

    fn handle_key_releases(keys: Vec<Key>, _state: &mut state::State) {        
        keys.iter().for_each(|key| match key {
            Key::W => println!("released w!"),
            Key::T => println!("released t!"),
            _ => (),
        });
    }
}

fn main() {
    let buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let hist_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let size = Point2{x: WIDTH as u32, y: HEIGHT as u32};
    let state = state::State { frame: frame::Frame{buffer, size },
                                     histogram: frame::Frame{buffer: hist_buffer, size },
                                     increment: 0 };

    let mut painter = painter::Painter {state, width: WIDTH, height: HEIGHT};

    painter.draw_window();
}