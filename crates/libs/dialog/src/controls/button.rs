use std::io::stdout;

use crossterm::cursor::MoveTo;
use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::style::Print;
use crossterm::QueueableCommand;
use tracing::info;

use crate::utils::Position;
use crate::{ButtonCount, DialogResult};
use crate::error::Result;

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

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn draw(&self) -> Result<()> {
        stdout()
            .queue(MoveTo(self.position.x as u16, self.position.y as u16))?
            .queue(Print(&self.name))?;

        Ok(())
    }

    pub fn handle_input(&self, code: KeyCode, _modifiers: KeyModifiers) -> Result<()> {
        if let KeyCode::Char(' ') = code {
            info!("Button clicked");
        }

        Ok(())
    }
    
}
