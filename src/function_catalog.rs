use crate::{algebra::Point2, constants::DOMAIN};


fn f1(x: Point2<f32>) -> Point2<f32> {(x + Point2{ x: DOMAIN.min.x, y: DOMAIN.min.y }) / 2.}
fn f2(x: Point2<f32>) -> Point2<f32> {(x + Point2{ x: DOMAIN.max.x, y: DOMAIN.min.y }) / 2.}
fn f3(x: Point2<f32>) -> Point2<f32> {(x + Point2{ x: DOMAIN.max.x, y: DOMAIN.max.y }) / 2.}

pub fn get_functions(name: &str) -> Vec<fn(Point2<f32>) -> Point2<f32>> {
    match name {
        "sierpinski" => vec![f1, f2, f3],
        _ => panic!("Non-existent function name")
    }
}