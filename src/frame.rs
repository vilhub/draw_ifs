use std::ops;

use crate::algebra::Point2;

#[derive(Debug, Copy, Clone)]
pub struct Pixel {
    pub r: u32,
    pub g: u32,
    pub b: u32,
    pub a: u32
}

pub struct Frame<T> {
    pub buffer: Vec<T>,
    pub size: Point2<u32>,
}

pub struct Domain {
    pub min: Point2<f32>,
    pub max: Point2<f32>,
}

impl<T> Frame<T> {
    #[allow(dead_code)]
    pub fn to_domain(&self, point: Point2<u32>, domain: Domain) -> Point2<f32> {
        let normalized_point = point.to_f32() / self.size.to_f32(); // In domain [0,1]^2
        normalized_point * (domain.max - domain.min) + domain.min
    }

    pub fn from_domain(&self, point: Point2<f32>, domain: Domain) -> Point2<u32> {
        let normalized_point = (point - domain.min) / (domain.max - domain.min); // In domain [0,1]^2
        (normalized_point * self.size.to_f32()).to_u32()
    }
}

impl Frame<Pixel> {
    pub fn increment_pixel(&mut self, point: Point2<u32>, color: Pixel) {
        let pixel_id = xy_to_id(point.x, point.y, self.size.x);
        if let Some(pixel) = self.buffer.get_mut(pixel_id as usize) {
            *pixel = *pixel + color;
        }
    }
}

pub fn id_to_xy(i: u32, x_size: u32) -> (u32, u32) {
    (i % x_size, i / x_size)
}

pub fn xy_to_id(x: u32, y: u32, x_size: u32) -> u32 {
    y * x_size + x
}

impl ops::Add for Pixel {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a
        }
    }
}

impl ops::Div<u32> for Pixel {
    type Output = Self;

    fn div(self, rhs: u32) -> Self {
        Self {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
            a: self.a / rhs
        }
    }
}
