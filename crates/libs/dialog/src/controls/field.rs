// region:    -- Fields
use crate::{error::Result, line_buffer::{LineBuffer, LineBufferColors}, utils::Position, DialogReturnValue, TextMode};

use std::io::stdout;

use crossterm::{cursor::{Hide, MoveTo, Show}, event::{KeyCode, KeyModifiers}, style::{Color, Colors, Print, SetColors}, QueueableCommand};

use super::UIElement;

#[derive(Debug, Clone)]
pub struct Field {
    name: String,
    display_len: usize,
    tab_index: Option<usize>,
    index: usize,
    value: String,
    position: Position,
    line_buffer: LineBuffer,
    label_colors: Colors
    
}

impl Default for Field {
    fn default() -> Self {
        Self {
            name: Default::default(),
            display_len: Default::default(),
            tab_index: Default::default(),
            index: Default::default(),
            value: Default::default(),
            position: Default::default(),
            line_buffer: Default::default(),
            label_colors: Colors::new(Color::White, Color::Black)
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldColors {
    label: Colors,
    input: LineBufferColors,
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

impl Field {
    pub fn new(name: impl Into<String>, display_len: usize, input_len: usize, tab_index: Option<usize>, index: usize) -> Self {
        Self {
            name: name.into(),
            display_len,
            tab_index,
            index,
            value: String::new(),
            line_buffer: LineBuffer::new(display_len, input_len, (0, 0).into(), '_'),
            position: (0, 0).into(),
            label_colors: Colors::new(Color::White, Color::Black),
        }
    }

    pub fn set_colors(&mut self, colors: FieldColors) {
        self.label_colors = colors.label;
        self.line_buffer.set_colors(colors.input);
    }

    fn draw_label(&self) -> Result<()> {
        stdout()
            .queue(SetColors(self.label_colors))?
            .queue(MoveTo(self.position.x as u16, self.position.y as u16))?
            .queue(Print(format!("{}: ", self.name)))?;

        Ok(())
    }

    pub fn get_field_index(&self) -> usize {
        self.index
    }

    pub fn get_display_window(&self) -> usize {
        self.display_len
    }

}

impl UIElement for Field {
    fn draw(&self) -> Result<()> {
        self.draw_label()?;
        self.line_buffer.draw()?;

        Ok(())
    }

    fn handle_input(&mut self, code: KeyCode, modifiers: KeyModifiers, mode: TextMode) -> Result<DialogReturnValue> {
        self.line_buffer.handle_input(code, modifiers, mode)?;
        self.value = self.line_buffer.buffer.clone();

        Ok(DialogReturnValue::default())
    }

    fn show_focus_indicator(&self, mode: TextMode) -> Result<()> {
        let pos = self.line_buffer.get_position();

        stdout()
            .queue(Show)?
            .queue(MoveTo(pos.x as u16, pos.y as u16))?;

        Ok(())
    }

    fn hide_focus_indicator(&mut self) -> Result<()> {
        self.line_buffer.set_pos(0);
        self.line_buffer.draw()?;
        stdout().queue(Hide)?;

        Ok(())
    }

    fn set_position(&mut self, position: Position) {
        self.position = position.clone();
        let pos = Position { x: position.x+self.name.len()+2, y: position.y };
        self.line_buffer.set_position(pos);
    }

    fn get_tab_index(&self) -> Option<usize> {
        self.tab_index
    }

    fn get_value(&self) -> Option<(String, String)> {
        Some((self.name.clone(), self.value.clone()))
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}