use std::time::SystemTime;

use minifb::Key;

mod painter;
mod state;

const WIDTH: usize = 640;
const HEIGHT: usize = 640;

impl painter::Paintable for painter::Painter {
    fn update_state(state: &mut state::State) {

        let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as u32 / 100;
        state.increment = time;
    
        let mut count: u32 = 0;
        for i in state.buffer.iter_mut() {
            let current_x = count % WIDTH as u32;
            let current_y = count / HEIGHT as u32;
            *i = ((current_x + state.increment) & 0xFF) << 8 | (current_y & 0xFF);
            count += 1;
        }
    }
    
    fn handle_key_presses(key: Vec<Key>, state: &mut state::State) {
        
    }
}

fn main() {
    let buffer: Vec<u32> = Vec::with_capacity(WIDTH * HEIGHT);
    let mut state = state::State { buffer, increment: 0 };

    let painter = painter::Painter {state, width: WIDTH, height: HEIGHT};

    painter.draw_window();
}


// window.get_keys().iter().for_each(|key| match key {
//     Key::W => println!("holding w!"),
//     Key::T => println!("holding t!"),
//     _ => (),
// });

// window.get_keys_released().iter().for_each(|key| match key {
//     Key::W => println!("released w!"),
//     Key::T => println!("released t!"),
//     _ => (),
// });