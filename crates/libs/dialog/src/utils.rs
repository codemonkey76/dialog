// region:    -- Position
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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


// region:    -- Icon

#[derive(Clone, Default)]
pub enum Icon {
    #[default]
    Info,
    Warning,
    Success,
    Error,
    Custom((String,Color))
}

impl std::fmt::Display for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Icon::Info => write!(f, "ℹ"),
            Icon::Warning => write!(f, "⚠"),
            Icon::Success => write!(f, "✔"),
            Icon::Error => write!(f, "❗"),
            Icon::Custom((icon_text, _)) => write!(f, "{icon_text}"),
        }
    }
}

// endregion: -- Icon

// region:    -- Color

#[derive(Clone, Debug)]
pub struct Color(crossterm::style::Color);

impl Default for Color {
    fn default() -> Self {
        Self(crossterm::style::Color::White)
    }
}

#[derive(Clone, Debug)]
pub struct Colors(crossterm::style::Colors);

impl Default for Colors {
    fn default() -> Self {
        Self(crossterm::style::Colors::new(crossterm::style::Color::White, crossterm::style::Color::Black))
    }
}

// endregion: -- Color