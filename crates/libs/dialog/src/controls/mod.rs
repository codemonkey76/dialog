use std::io::stdout;

use crossterm::{cursor::MoveTo, event::{KeyCode, KeyModifiers}, QueueableCommand};

use crate::{error::Result, DialogSpecs};
pub mod field;
pub mod button;

// region:    -- Focusable
pub trait Focusable {
    fn get_tab_index(&self) -> Option<usize>;
    fn focus(&self, specs: Option<DialogSpecs>) -> Result<()>;
    fn handle_input(&mut self, code: KeyCode, modifiers: KeyModifiers) -> Result<()>;
}
// endregion: -- Focusable


#[derive(Debug, Clone)]
pub enum Control {
    TextField(field::Field),
    Button(button::Button)
}

impl Control {
    pub fn draw(&self) -> Result<()> {
        match self {
            Control::TextField(field) => field.draw()?,
            Control::Button(button) => button.draw()?
        };

        Ok(())
    }
}
            


            // let (pos, label) = match self {
            //     Control::TextField(field) => {
            //         let x = (specs.position.x + 1 + specs.margin.x + specs.max_name_len - field.name.len()) as u16;
            //         let y = (specs.position.y + 1 + specs.margin.y + 2*field.index) as u16;
                    
            //         ((x, y), format!("{}: ", field.name))
            //     }
                    
            //     Control::Button(button) => {
            //         let x = match (specs.button_count, button.index) {
            //             (1, _) => (specs.position.x + specs.size.width / 2 - button.name.len() / 2) as u16,
            //             (2, 0) => (specs.position.x + specs.margin.x) as u16,
            //             (2, 1) => (specs.position.x + specs.size.width - specs.margin.x - button.name.len())  as u16,
            //             (3, 0) => (specs.position.x + specs.margin.x) as u16,
            //             (3, 1) => (specs.position.x + specs.size.width / 2 - button.name.len() / 2) as u16,
            //             (3, 2) => (specs.position.x + specs.size.width - specs.margin.x - button.name.len()) as u16,
            //             _ => panic!("Invalid button index: {} or button count: {}", button.index, specs.button_count)
            //         };
            //         let y = (specs.position.y + specs.size.height - 2) as u16;

            //         ((x, y), format!("{}", button.name))
            //     },
            // };

            // stdout()
            //     .queue(MoveTo(pos.0, pos.1))?
            //     .queue(Print(label))?
            //     .flush()?;
            

impl Focusable for Control {
    fn get_tab_index(&self) -> Option<usize> {
        match self {
            Control::TextField(field) => field.tab_index,
            Control::Button(button) => button.tab_index,
        }
    }

    fn focus(&self, specs: Option<DialogSpecs>) -> Result<()> {
        if let Some(specs) = specs {
            let (x, y) = match self {
                Control::TextField(field) => {
                    let x = (specs.position.x + 1 + specs.margin.x + specs.max_name_len + 2) as u16;
                    let y = (specs.position.y + 1 + specs.margin.y + 2 * field.index) as u16;

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
                button.handle_input(code, modifiers)?
            }
        };

        Ok(())
    }
}