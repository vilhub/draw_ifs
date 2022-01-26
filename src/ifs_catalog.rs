use crate::{
    algebra::{Matrix2, Point2},
    constants::DOMAIN,
    ifs_computer::IFSComputer,
};

fn r(p: Point2<f32>) -> f32 {(p.x * p.x + p.y * p.y).sqrt()}

fn f1(x: Point2<f32>) -> Point2<f32> {(x + Point2{ x: DOMAIN.min.x, y: DOMAIN.min.y }) / 2.}
fn f2(x: Point2<f32>) -> Point2<f32> {(x + Point2{ x: DOMAIN.max.x, y: DOMAIN.min.y }) / 2.}
fn f3(x: Point2<f32>) -> Point2<f32> {(x + Point2{ x: DOMAIN.max.x, y: DOMAIN.max.y }) / 2.}

fn spherical(x: Point2<f32>) -> Point2<f32> {
    let squared = x * x;
    let r_squared = squared.x + squared.y;
    x / r_squared
}

fn horseshoe(p: Point2<f32>) -> Point2<f32> {Point2{ x: (p.x - p.y)*(p.x + p.y), y: 2. * p.x * p.y } / r(p)}

fn fern_1(x: Point2<f32>) -> Point2<f32> { Matrix2{a: 0., b: 0., c: 0., d: 0.16} * x + Point2{ x: 0., y: 0. }}
fn fern_2(x: Point2<f32>) -> Point2<f32> { Matrix2{a: 0.85, b: 0.04, c: -0.04, d: 0.85} * x + Point2{ x: 0., y: 1.6 }}
fn fern_3(x: Point2<f32>) -> Point2<f32> { Matrix2{a: 0.2, b: -0.26, c: 0.23, d: 0.22} * x + Point2{ x: 0., y: 1.6 }}
fn fern_4(x: Point2<f32>) -> Point2<f32> { Matrix2{a: -0.15, b: 0.28, c: 0.26, d: 0.24} * x + Point2{ x: 0., y: 0.44 }}

pub fn get_ifs_preset(name: &str) -> IFSComputer {
    match name {
        "sierpinski" => IFSComputer {functions: vec![f1, f2, f3],
                                     weights: vec![1., 1., 1.]},
        "fern" => IFSComputer {functions: vec![fern_1, fern_2, fern_3, fern_4],
                                     weights: vec![0.01, 0.85, 0.07, 0.07]},
        "test" => IFSComputer {functions: vec![f1, f2, spherical, horseshoe],
                               weights: vec![1., 1., 1., 1.]},
        _ => panic!("Non-existent function name")
    }
}