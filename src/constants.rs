use crate::{algebra::Point2, frame};

pub(crate) const WIDTH: usize = 800;
pub(crate) const HEIGHT: usize = 800;

pub(crate) const DOMAIN: frame::Domain = frame::Domain {
    min: Point2 { x: -2., y: -2. },
    max: Point2 { x: 2., y: 2. },
};

pub(crate) const SUPERSAMPLING: u32 = 1;