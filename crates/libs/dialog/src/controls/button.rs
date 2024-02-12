use std::io::stdout;

use crossterm::cursor::MoveTo;
use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::style::Print;
use crossterm::QueueableCommand;
use tracing::info;

use crate::utils::Position;
use crate::{ButtonCount, DialogResult, DialogReturnValue};
use crate::error::Result;

use super::UIElement;

#[derive(Debug, Default, Clone)]
pub struct Button {
    pub name: String,
    pub tab_index: Option<usize>,
    pub result: DialogResult,
    pub index: ButtonCount,
    pub position: Position
}

impl Button {
    pub fn new(name: impl Into<String>, tab_index: Option<usize>, result: DialogResult, index: ButtonCount) -> Self {
        Self {
            name: name.into(),
            tab_index,
            result,
            index,
            position: Position::default()
         }
    }
}

impl UIElement for Button {
    fn draw(&self) -> Result<()> {
        stdout()
            .queue(MoveTo(self.position.x as u16, self.position.y as u16))?
            .queue(Print(&self.name))?;

        Ok(())
    }

    fn handle_input(&mut self, code: KeyCode, _: KeyModifiers) -> Result<DialogReturnValue> {
        if let KeyCode::Char(' ') = code {
            Ok(DialogReturnValue {
                should_quit: true,
                dialog_result: Some(self.result.clone()),
            })
        } else {
            Ok(DialogReturnValue::default())
        }
    }

    fn show_focus_indicator(&self) -> Result<()> {
        stdout()
                .queue(MoveTo(self.position.x as u16 - 2, self.position.y as u16))?
                .queue(Print("<"))?
                .queue(MoveTo((self.position.x + self.name.len() + 1) as u16, self.position.y as u16))?
                .queue(Print(">"))?;

        Ok(())
    }

    fn hide_focus_indicator(&mut self) -> Result<()> {
        stdout()
                .queue(MoveTo(self.position.x as u16 - 2, self.position.y as u16))?
                .queue(Print(" "))?
                .queue(MoveTo((self.position.x + self.name.len() + 1) as u16, self.position.y as u16))?
                .queue(Print(" "))?;

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

