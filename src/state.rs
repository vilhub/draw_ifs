use crate::frame::Frame;

pub struct State {
    pub frame: Frame,
    pub histogram: Frame,
    pub increment: u32
}