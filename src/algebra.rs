use std::ops;

use rand::{distributions::Standard, prelude::Distribution, Rng};

#[derive(Debug, Copy, Clone)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Copy, Clone)]
pub struct Matrix2<T> {
    // Matrix |a, b|
    //        |c, d|
    pub a: T,
    pub b: T,
    pub c: T,
    pub d: T,
}

impl<T> Distribution<Point2<T>> for Standard
where
    Standard: Distribution<T>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point2<T> {
        Point2 {
            x: rng.gen(),
            y: rng.gen(),
        }
    }
}

impl Point2<u32> {
    pub fn to_f32(self) -> Point2<f32> {
        Point2 {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}

impl Point2<f32> {
    pub fn to_u32(self) -> Point2<u32> {
        Point2 {
            x: self.x as u32,
            y: self.y as u32,
        }
    }
}

impl<T: ops::Add<Output = T>> ops::Add for Point2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Point2<T> {
        Point2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: ops::Add<Output = T> + Copy + Clone> ops::Add<T> for Point2<T> {
    type Output = Self;

    fn add(self, rhs: T) -> Point2<T> {
        Point2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl<T: ops::Sub<Output = T>> ops::Sub for Point2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Point2<T> {
        Point2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: ops::Sub<Output = T> + Copy + Clone> ops::Sub<T> for Point2<T> {
    type Output = Self;

    fn sub(self, rhs: T) -> Point2<T> {
        Point2 {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl<T: ops::Mul<Output = T>> ops::Mul<Point2<T>> for Point2<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Point2<T> {
        Point2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T: ops::Mul<Output = T> + Copy + Clone> ops::Mul<T> for Point2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Point2<T> {
        Point2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: ops::Div<Output = T>> ops::Div<Point2<T>> for Point2<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Point2<T> {
        Point2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T: ops::Div<Output = T> + Copy + Clone> ops::Div<T> for Point2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Point2<T> {
        Point2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> ops::Mul<Point2<T>> for Matrix2<T> {
    type Output = Point2<T>;

    fn mul(self, rhs: Point2<T>) -> Point2<T> {
        Point2 {
            x: self.a * rhs.x + self.b * rhs.y,
            y: self.c * rhs.x + self.d * rhs.y,
        }
    }
}
