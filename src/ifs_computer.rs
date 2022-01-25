use std::sync::Mutex;

use image::{GrayImage, Luma};
use minifb::Key;
use rand::{
    distributions::{Distribution, WeightedIndex},
    Rng,
};

use crate::{
    algebra::Point2,
    constants::{DOMAIN, SUPERSAMPLING, PALETTE},
    frame::{id_to_xy, xy_to_id, Frame, Pixel},
};

pub struct IFSComputer {
    pub functions: Vec<fn(Point2<f32>) -> Point2<f32>>,
    pub weights: Vec<f32>,
}

impl IFSComputer {
    pub fn compute_ifs(&self, histogram: &Mutex<Frame<Pixel>>) {
        let mut rng = rand::thread_rng();
        let mut iter_point = rng.gen();
        iter_point = (iter_point - 0.5) * 2.;
        let dist = WeightedIndex::new(&self.weights).unwrap();
        for i in 1..1000000000 {
            let rand_num = dist.sample(&mut rng);
            iter_point = self.functions[rand_num](iter_point);
            if i > 20 && DOMAIN.is_in_domain(&iter_point) {
                let mut histogram_guard = histogram.lock().unwrap();
                let frame_point = histogram_guard.from_domain(iter_point, DOMAIN);
                histogram_guard.increment_pixel(frame_point, PALETTE[rand_num]);
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

pub fn draw_on_frame(current_frame: &mut Frame<u32>, histogram: &Frame<Pixel>) {
    const USE_IMAGE_BUFFER: bool = false;
    if USE_IMAGE_BUFFER {
        // Branch only kept as a reference to see how to manipulate images with imageproc
        let mut temp_image = GrayImage::new(histogram.size.x, histogram.size.y);
        for (i, j) in histogram.buffer.iter().enumerate() {
            let histogram_value = j.a;
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
        for (i_draw_pixel, draw_pixel) in current_frame.buffer.iter_mut().enumerate() {
            // Resize SUPERSAMPLING x SUPERSAMPLING pixel squares to one pixel
            let mut pixel_average = Pixel {r: 0, g: 0, b: 0, a: 0};
            let (draw_x, draw_y) = id_to_xy(i_draw_pixel as u32, current_frame.size.x);
            for k in 0..SUPERSAMPLING {
                for l in 0..SUPERSAMPLING {
                    let (super_x, super_y) =
                        (SUPERSAMPLING * draw_x + k, SUPERSAMPLING * draw_y + l);
                    let super_id = xy_to_id(super_x, super_y, histogram.size.x);
                    let pixel_value = histogram.buffer.get(super_id as usize);
                    pixel_average = pixel_average + *pixel_value.unwrap();
                }
            }
            pixel_average = pixel_average / (SUPERSAMPLING * SUPERSAMPLING);
            let histogram_value = pixel_average.a;
            let max_value: f32 = 255.;
            let scaling_factor = ((1 + histogram_value) as f32).log2() / max_value.log2();
            let r = (pixel_average.r as f32 / histogram_value as f32 * scaling_factor) as u32;
            let g = (pixel_average.g as f32 / histogram_value as f32 * scaling_factor) as u32;
            let b = (pixel_average.b as f32 / histogram_value as f32 * scaling_factor) as u32;
            let intensity_r = std::cmp::min(r, 0xFF);
            let intensity_g = std::cmp::min(g, 0xFF);
            let intensity_b = std::cmp::min(b, 0xFF);
            *draw_pixel = intensity_r << 16 | intensity_g << 8 | intensity_b;
        }
    }
}
