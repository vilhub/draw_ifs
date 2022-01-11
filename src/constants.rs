use crate::{frame, algebra::Point2};


pub(crate) const WIDTH: usize = 640;
pub(crate) const HEIGHT: usize = 640;

pub(crate) const DOMAIN: frame::Domain = frame::Domain { min: Point2{x: -1., y:-1.},
                                                         max: Point2{x: 1., y: 1.} };