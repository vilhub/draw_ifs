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

fn main() {
    let hist_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let size = Point2{x: WIDTH as u32, y: HEIGHT as u32};

    let ifs_computer = IFSComputer { histogram: frame::Frame{buffer: hist_buffer, size } };
    draw_window(ifs_computer);
}