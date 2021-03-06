use crate::frame_buffer::Color;
use crate::lines::*;

#[derive(Debug, Copy, Clone)]
pub enum BarrierKind {
    Basic,
}

#[derive(Debug, Copy, Clone)]
pub struct Barrier {
    pub seg: Segment,
    pub kind: BarrierKind,
    pub color: Color,
}
