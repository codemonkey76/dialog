// region:    -- Position
#[derive(Debug, Clone, Default)]
pub struct Position {
    pub x: u16,
    pub y: u16
}

impl From<(u16, u16)> for Position {
    fn from(value: (u16, u16)) -> Self {
        Self { x: value.0, y: value.1 }
    }
}

// endregion: -- Position

// region:    -- Size

#[derive(Debug, Clone, Default)]
pub struct Size {
    pub width: u16,
    pub height: u16
}

impl From<(u16, u16)> for Size {
    fn from(value: (u16, u16)) -> Self {
        Self { width: value.0, height: value.1 }
    }
}
// endregion: -- Size