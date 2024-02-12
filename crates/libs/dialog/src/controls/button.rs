use std::io::stdout;

use crossterm::cursor::MoveTo;
use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::style::{Color, Colors, Print, SetColors};
use crossterm::QueueableCommand;

use crate::utils::Position;
use crate::{ButtonCount, DialogResult, DialogReturnValue, TextMode};
use crate::error::Result;

use super::UIElement;


#[derive(Debug, Clone)]
pub struct ButtonColors {
    button: Colors,
    focus: Colors,
}

impl ButtonColors {
    pub fn new(button: Colors, focus: Colors) -> Self {
        Self {
            button, focus 
        }
    }
}


#[derive(Debug, Clone)]
pub struct Button {
    pub name: String,
    pub tab_index: Option<usize>,
    pub result: DialogResult,
    pub index: ButtonCount,
    pub position: Position,
    pub colors: ButtonColors
}

impl Default for ButtonColors {
    fn default() -> Self {
        Self {
            button: Colors::new(Color::White, Color::Black),
            focus: Colors::new(Color::White, Color::Black)
        }
    }
}

impl Button {
    pub fn new(name: impl Into<String>, tab_index: Option<usize>, result: DialogResult, index: ButtonCount) -> Self {
        Self {
            name: name.into(),
            tab_index,
            result,
            index,
            position: Position::default(),
            colors: ButtonColors::default()
         }
    }

    pub fn set_colors(&mut self, colors: ButtonColors) {
        self.colors = colors;
    }
}

impl UIElement for Button {
    fn draw(&self) -> Result<()> {
        stdout()
            .queue(SetColors(self.colors.button))?
            .queue(MoveTo(self.position.x as u16, self.position.y as u16))?
            .queue(Print(&self.name))?;

        Ok(())
    }

    fn handle_input(&mut self, code: KeyCode, _: KeyModifiers, _: TextMode) -> Result<DialogReturnValue> {
        if let KeyCode::Char(' ') = code {
            Ok(DialogReturnValue {
                should_quit: true,
                dialog_result: Some(self.result.clone()),
            })
        } else {
            Ok(DialogReturnValue::default())
        }
    }

    fn show_focus_indicator(&self, _: TextMode) -> Result<()> {
        stdout()
                .queue(SetColors(self.colors.focus))?
                .queue(MoveTo(self.position.x as u16 - 2, self.position.y as u16))?
                .queue(Print("< "))?
                .queue(MoveTo((self.position.x + self.name.len()) as u16, self.position.y as u16))?
                .queue(Print(" >"))?;

        Ok(())
    }

    fn hide_focus_indicator(&mut self) -> Result<()> {
        stdout()
            .queue(SetColors(self.colors.focus))?
            .queue(MoveTo(self.position.x as u16 - 2, self.position.y as u16))?
            .queue(Print("  "))?
            .queue(MoveTo((self.position.x + self.name.len()) as u16, self.position.y as u16))?
            .queue(Print("  "))?;

        Ok(())
    }

    fn set_position(&mut self, position: Position) {
        self.position = position
    }

    fn get_tab_index(&self) -> Option<usize> {
        self.tab_index
    }

    fn get_value(&self) -> Option<(String, String)> {
        None
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

