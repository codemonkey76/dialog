use crossterm::event::{KeyCode, KeyModifiers};

use crate::{error::Result, utils::Position, DialogReturnValue};
pub mod field;
pub mod button;


#[derive(Debug, Clone)]
pub enum Control {
    TextField(field::Field),
    Button(button::Button)
}

impl UIElement for Control {
    fn draw(&self) -> Result<()> {
        match self {
            Control::TextField(f) => f.draw()?,
            Control::Button(b) => b.draw()?
        };
        
        Ok(())
    }

    fn handle_input(&mut self, code: KeyCode, modifiers: KeyModifiers) -> Result<DialogReturnValue> {
        match self {
            Control::TextField(f) => f.handle_input(code, modifiers),
            Control::Button(b) => b.handle_input(code, modifiers)
        }
    }

    fn show_focus_indicator(&self) -> Result<()> {
        match self {
            Control::TextField(f) => f.show_focus_indicator()?,
            Control::Button(b) => b.show_focus_indicator()?
        }

        Ok(())
    }

    fn hide_focus_indicator(&mut self) -> Result<()> {
        match self {
            Control::TextField(f) => f.hide_focus_indicator()?,
            Control::Button(b) => b.hide_focus_indicator()?
        };

        Ok(())
    }

    fn set_position(&mut self, position: Position) {
        match self {
            Control::TextField(f) => f.set_position(position),
            Control::Button(b) => b.set_position(position)
        };
    }

    fn get_tab_index(&self) -> Option<usize> {
        match self {
            Control::TextField(f) => f.get_tab_index(),
            Control::Button(b) => b.get_tab_index()
        }
    }

    fn get_value(&self) -> Option<(String, String)> {
        match self {
            Control::TextField(f) => f.get_value(),
            Control::Button(b) => b.get_value()
        }
    }

    fn get_name(&self) -> String {
        match self {
            Control::TextField(f) => f.get_name(),
            Control::Button(b) => b.get_name()
        }
    }
}

pub trait UIElement {
    fn draw(&self) -> Result<()>;
    fn handle_input(&mut self, code: KeyCode, modifiers: KeyModifiers) -> Result<DialogReturnValue>;
    fn show_focus_indicator(&self) -> Result<()>;
    fn hide_focus_indicator(&mut self) -> Result<()>;
    fn set_position(&mut self, position: Position);
    fn get_tab_index(&self) -> Option<usize>;
    fn get_name(&self) -> String;
    fn get_value(&self) -> Option<(String, String)>;
}