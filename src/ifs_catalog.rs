use crate::{algebra::Point2, constants::DOMAIN, ifs_computer::IFSComputer};


fn f1(x: Point2<f32>) -> Point2<f32> {(x + Point2{ x: DOMAIN.min.x, y: DOMAIN.min.y }) / 2.}
fn f2(x: Point2<f32>) -> Point2<f32> {(x + Point2{ x: DOMAIN.max.x, y: DOMAIN.min.y }) / 2.}
fn f3(x: Point2<f32>) -> Point2<f32> {(x + Point2{ x: DOMAIN.max.x, y: DOMAIN.max.y }) / 2.}

fn spherical(x: Point2<f32>) -> Point2<f32> {
    let squared = x * x;
    let r_squared = squared.x + squared.y;
    x / r_squared
}

pub fn get_ifs_preset(name: &str) -> IFSComputer {
    match name {
        "sierpinski" => IFSComputer {functions: vec![f1, f2, f3],
                                     weights: vec![1., 1., 1.]},
        "test" => IFSComputer {functions: vec![f1, f2, f3, spherical],
                               weights: vec![1., 1., 1., 3.]},
        _ => panic!("Non-existent function name")
    }
}