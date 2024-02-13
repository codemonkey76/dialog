use crossterm::style::{Color, Colors};

#[derive(Debug)]
pub struct DialogColors {
    pub(crate) border: Colors,
    pub(crate) fill: Colors,
    pub(crate) overlay: Colors,
    pub(crate) fields: FieldColors,
    pub(crate) buttons: ButtonColors
}

impl Default for DialogColors {
    fn default() -> Self {
        Self {
            border: Colors::new(Color::White, Color::Black),
            fill: Colors::new(Color::White, Color::Black),
            overlay: Colors::new(Color::White, Color::Black),
            fields: Default::default(),
            buttons: Default::default()
        }
    }
}

impl DialogColors {
    pub fn new(
        border: Colors,
        fill: Colors,
        overlay: Colors,
        labels: Colors,
        inputs: Colors,
        input_indicators: Colors,
        buttons: Colors,
        button_focus: Colors
    ) -> Self {
        Self {
            border,
            fill,
            overlay,
            fields: FieldColors::new(labels, inputs, input_indicators),
            buttons: ButtonColors::new(buttons, button_focus),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldColors {
    pub(crate) label: Colors,
    pub(crate) input: LineBufferColors,
}
impl FieldColors {
    pub fn new(label: Colors, input: Colors, indicators: Colors) -> Self {
        Self {
            label,
            input: LineBufferColors::new(input, indicators)
        }
    }
}

impl Default for FieldColors {
    fn default() -> Self {
        Self {
            label: Colors::new(Color::White, Color::Black),
            input: Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct LineBufferColors {
    pub(crate) focus: Colors,
    pub(crate) input: Colors
}

impl LineBufferColors {
    pub fn new(input: Colors, indicators: Colors) -> Self {
        Self {
            focus: indicators,
            input,
        }
    }
}
impl Default for LineBufferColors {
    fn default() -> Self {
        Self {
            focus: Colors::new(Color::White, Color::Black),
            input: Colors::new(Color::White, Color::Black)
        }
    }
}


#[derive(Debug, Clone)]
pub struct ButtonColors {
    pub(crate) button: Colors,
    pub(crate) focus: Colors,
}

impl ButtonColors {
    pub fn new(button: Colors, focus: Colors) -> Self {
        Self {
            button, focus 
        }
    }
}