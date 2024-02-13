// region:    -- Position
#[derive(Debug, Clone, Default)]
pub struct Position {
    pub(crate) x: usize,
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
pub(crate) struct Size {
    pub(crate) width: usize,
    pub(crate) height: usize
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