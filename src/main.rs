use algebra::Point2;
use ifs_computer::IFSComputer;
use window_drawer::draw_window;
use function_catalog::get_functions;

mod function_catalog;
mod constants;
mod window_drawer;
mod frame;
mod algebra;
mod ifs_computer;

fn main() {

    let ifs_computer = IFSComputer { functions: get_functions("sierpinski") };
    draw_window(ifs_computer);
}