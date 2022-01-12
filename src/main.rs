use algebra::Point2;
use window_drawer::draw_window;
use ifs_catalog::get_ifs_preset;

mod ifs_catalog;
mod constants;
mod window_drawer;
mod frame;
mod algebra;
mod ifs_computer;

fn main() {

    let ifs_computer = get_ifs_preset("fern");
    draw_window(ifs_computer);
}