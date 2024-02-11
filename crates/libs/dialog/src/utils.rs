// region:    -- Position
#[derive(Debug, Clone, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Self { x: value.0, y: value.1 }
    }
}

// endregion: -- Position

// region:    -- Size

#[derive(Debug, Clone, Default)]
pub struct Size {
    pub width: usize,
    pub height: usize
}

impl From<(usize, usize)> for Size {
    fn from(value: (usize, usize)) -> Self {
        Self { width: value.0, height: value.1 }
    }
}

impl From<(u16, u16)> for Size {
    fn from(value: (u16, u16)) -> Self {
        Self { width: value.0 as usize, height: value.1 as usize }
    }
}
// endregion: -- Size