use algebra::Point2;
use ifs_catalog::get_ifs_preset;
use window_drawer::draw_window;

mod algebra;
mod constants;
mod frame;
mod ifs_catalog;
mod ifs_computer;
mod window_drawer;

fn main() {
    let ifs_computer = get_ifs_preset("test");
    draw_window(ifs_computer);
}
