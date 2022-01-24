use crate::{
    algebra::Point2,
    frame::{Domain, Pixel},
};

pub(crate) const WIDTH: usize = 800;
pub(crate) const HEIGHT: usize = 800;

pub(crate) const DOMAIN: Domain = Domain {
    min: Point2 { x: -2., y: -2. },
    max: Point2 { x: 2., y: 2. },
};

pub(crate) const SUPERSAMPLING: u32 = 1;

pub(crate) const PALETTE: &'static [Pixel] = &[
    Pixel {
        r: 255,
        g: 0,
        b: 0,
        a: 1
    },
    Pixel {
        r: 0,
        g: 255,
        b: 0,
        a: 1
    },
    Pixel {
        r: 0,
        g: 0,
        b: 255,
        a: 1
    },
    Pixel {
        r: 255,
        g: 255,
        b: 255,
        a: 1
    }
];
