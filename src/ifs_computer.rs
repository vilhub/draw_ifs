use std::sync::Mutex;

use image::{GrayImage, Luma};
use minifb::Key;
use rand::{
    distributions::{Distribution, WeightedIndex},
    Rng,
};

use crate::{algebra::Point2, constants::DOMAIN, frame::Frame};

pub struct IFSComputer {
    pub functions: Vec<fn(Point2<f32>) -> Point2<f32>>,
    pub weights: Vec<f32>,
}

impl IFSComputer {
    pub fn compute_ifs(&self, histogram: &Mutex<Frame>) {
        let mut rng = rand::thread_rng();
        let mut iter_point = rng.gen();
        iter_point = (iter_point - 0.5) * 2.;
        let dist = WeightedIndex::new(&self.weights).unwrap();
        for i in 1..1000000000 {
            let rand_num = dist.sample(&mut rng);
            iter_point = self.functions[rand_num](iter_point);
            if i > 20 {
                let mut histogram_guard = histogram.lock().unwrap();
                let frame_point = histogram_guard.from_domain(iter_point, DOMAIN);
                histogram_guard.increment_pixel(frame_point);
            }
        }
    }
}

pub fn handle_key_presses(keys: Vec<Key>) {
    keys.iter().for_each(|key| match key {
        Key::W => println!("holding w!"),
        Key::T => println!("holding t!"),
        _ => (),
    });
}

pub fn handle_key_releases(keys: Vec<Key>) {
    keys.iter().for_each(|key| match key {
        Key::W => println!("released w!"),
        Key::T => println!("released t!"),
        _ => (),
    });
}

pub fn draw_on_frame(current_frame: &mut Frame, histogram: &Frame) {
    const USE_IMAGE_BUFFER: bool = false;
    if USE_IMAGE_BUFFER {
        let mut temp_image = GrayImage::new(histogram.size.x, histogram.size.y);
        for (i, j) in histogram.buffer.iter().enumerate() {
            let histogram_value = *j;
            let scaling: f32 = 255.;
            let normalized_value = ((1 + histogram_value) as f32).log2() * scaling / scaling.log2();
            let intensity = std::cmp::min(normalized_value as u32, 0xFF);
            temp_image.put_pixel(
                i as u32 % histogram.size.x,
                i as u32 / histogram.size.x as u32,
                Luma([intensity as u8]),
            );
        }
        let temp_buffer = temp_image.as_raw();
        for (i, j) in current_frame.buffer.iter_mut().zip(temp_buffer.iter()) {
            let pixel = *j as u32;
            *i = pixel << 16 | pixel << 8 | pixel;
        }
    } else {
        for (i, j) in current_frame.buffer.iter_mut().zip(histogram.buffer.iter()) {
            let histogram_value = *j;
            let scaling: f32 = 255.;
            let normalized_value = ((1 + histogram_value) as f32).log2() * scaling / scaling.log2();
            let intensity = std::cmp::min(normalized_value as u32, 0xFF);
            *i = intensity << 16 | intensity << 8 | intensity;
        }
    }
}
