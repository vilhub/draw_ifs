use minifb::Key;
use rand::Rng;

use crate::{frame::Frame, algebra::Point2, DOMAIN};


pub struct IFSComputer {
    pub histogram: Frame
}

impl IFSComputer {
    pub fn compute_ifs(&mut self) {
        let mut rng = rand::thread_rng();
        let mut iter_point = rng.gen();
        iter_point = (iter_point - 0.5) * 2.;
        for i in 1..10000 {
            match rng.gen_range(1..=3) { // Optimize sampling
                1 => iter_point = (iter_point + Point2{ x: DOMAIN.min.x, y: DOMAIN.min.y }) / 2.,
                2 => iter_point = (iter_point + Point2{ x: DOMAIN.max.x, y: DOMAIN.min.y }) / 2.,
                3 => iter_point = (iter_point + Point2{ x: DOMAIN.max.x, y: DOMAIN.max.y }) / 2.,
                _ => ()
            }
            if i > 20 {
                let frame_point = self.histogram.from_domain(iter_point, DOMAIN);
                self.histogram.increment_pixel(frame_point);
            }
        }
    }

    pub fn update_state(&mut self) {
        self.compute_ifs();
    }
    
    pub fn handle_key_presses(&self, keys: Vec<Key>) {        
        keys.iter().for_each(|key| match key {
            Key::W => println!("holding w!"),
            Key::T => println!("holding t!"),
            _ => (),
        });
    }

    pub fn handle_key_releases(&self, keys: Vec<Key>) {        
        keys.iter().for_each(|key| match key {
            Key::W => println!("released w!"),
            Key::T => println!("released t!"),
            _ => (),
        });
    }

    pub fn draw_on_frame(&self, current_frame: &mut Frame){
        for (i, j) in current_frame.buffer.iter_mut().zip(self.histogram.buffer.iter()) {
            let intensity = std::cmp::min(*j, 0xFF);
            *i = intensity << 16 | intensity << 8 | intensity;
        }
    }
}