#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum PipeDirection {
    Top,
    Bottom,
    Left,
    Right,
}

impl PipeDirection {
    pub fn invert(&self) -> PipeDirection {
        match self {
            PipeDirection::Top => PipeDirection::Bottom,
            PipeDirection::Bottom => PipeDirection::Top,
            PipeDirection::Left => PipeDirection::Right,
            PipeDirection::Right => PipeDirection::Left,
        }
    }
}
