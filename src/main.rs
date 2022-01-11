use algebra::Point2;
use ifs_computer::IFSComputer;
use window_drawer::draw_window;

mod window_drawer;
mod frame;
mod algebra;
mod ifs_computer;

const WIDTH: usize = 640;
const HEIGHT: usize = 640;

const DOMAIN: frame::Domain = frame::Domain { min: Point2{x: -1., y:-1.},
                                              max: Point2{x: 1., y: 1.} };

fn f1(x: Point2<f32>) -> Point2<f32> {(x + Point2{ x: DOMAIN.min.x, y: DOMAIN.min.y }) / 2.}
fn f2(x: Point2<f32>) -> Point2<f32> {(x + Point2{ x: DOMAIN.max.x, y: DOMAIN.min.y }) / 2.}
fn f3(x: Point2<f32>) -> Point2<f32> {(x + Point2{ x: DOMAIN.max.x, y: DOMAIN.max.y }) / 2.}

fn main() {

    let ifs_computer = IFSComputer { functions: vec! [f1, f2, f3] };
    draw_window(ifs_computer);
}