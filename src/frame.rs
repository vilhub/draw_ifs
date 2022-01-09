use crate::algebra::Point2;


pub struct Frame {
    buffer: Vec<u32>,
    size: Point2<u32>
}

pub struct Domain {
    min: Point2<f32>,
    max: Point2<f32>
}

impl Frame {
    pub fn to_domain(&self, point: Point2<u32>, domain: Domain) -> Point2<f32> {
        let normalized_point = point.to_f32() / self.size.to_f32(); // In domain [0,1]^2
        normalized_point * (domain.max - domain.min) + domain.min 
    }

    pub fn from_domain(&self, point: Point2<f32>, domain: Domain) -> Point2<u32> {
        let normalized_point = (point - domain.min) / (domain.max - domain.min); // In domain [0,1]^2
        (normalized_point * self.size.to_f32()).to_u32() 
    }
}