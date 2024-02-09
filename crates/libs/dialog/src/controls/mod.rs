use std::io::stdout;

use crossterm::{cursor::MoveTo, event::{KeyCode, KeyModifiers}, QueueableCommand};

use crate::{error::Result, DialogSpecs};
pub mod field;
pub mod button;



// region:    -- Focusable
pub trait Focusable {
    fn get_tab_index(&self) -> Option<u8>;
    fn focus(&self, specs: Option<DialogSpecs>) -> Result<()>;
    fn handle_input(&mut self, code: KeyCode, modifiers: KeyModifiers) -> Result<()>;
}
// endregion: -- Focusable


#[derive(Debug, Clone)]
pub enum Control<'a> {
    TextField(&'a field::Field),
    Button(&'a button::Button)
}

impl<'a> Focusable for Control<'a> {
    fn get_tab_index(&self) -> Option<u8> {
        match self {
            Control::TextField(field) => field.tab_index,
            Control::Button(button) => button.tab_index,
        }
    }

    fn focus(&self, specs: Option<DialogSpecs>) -> Result<()> {
        if let Some(specs) = specs {
            let (x, y) = match self {
                Control::TextField(field) => {
                    let x = specs.position.x + 1 + specs.margin.x + specs.max_name_len as u16 + 2;
                    let y = specs.position.y + 1 + specs.margin.y + 2 * field.index;

                    (x, y)
                }

                Control::Button(_button) => {
                    (0, 0)
                }
            };

            stdout().queue(MoveTo(x, y))?;
        }
        Ok(())
    }

    fn handle_input(&mut self, code: KeyCode, modifiers: KeyModifiers) -> Result<()> {
        
        match self {
            Control::TextField(field) => {
                field.handle_text_input(code, modifiers)?;
            }
            Control::Button(button) => {

            }
        };

        Ok(())
    }
}