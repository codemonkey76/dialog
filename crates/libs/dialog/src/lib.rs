use std::io::{stdout, Write};

use crossterm::{cursor::MoveTo, style::Print, terminal::size, QueueableCommand};

use crate::error::Result;

use crate::{utils::{Position, Size, Colors}, controls::{Fields, Buttons}, borders::{BorderChars, Borders}};

pub mod borders;
pub mod controls;
mod utils;
pub mod error;


#[derive(Debug)]
pub struct DialogSpecs {
    pub position: Position,
    pub size: Size,
    pub margin: u16
}

#[derive(Debug, Default)]
pub struct Dialog {
    pub position: Option<Position>,
    pub size: Option<Size>,

    pub screen_size: Option<Size>,

    title: String,
    fields: Fields,
    buttons: Buttons,
    border_chars: BorderChars,
    margin: u16
}



impl Dialog {
    pub fn resize(&mut self) -> Result<()> {
        self.calc_size();
        self.calc_screen_size()?;
        self.calc_pos();

        Ok(())
    }
    
    fn draw_border(&self) -> Result<()> {

        if let (Some(size), Some(pos)) = (&self.size, &self.position) {
        // region:    -- Top Row
            
        stdout()
           .queue(MoveTo(pos.x, pos.y))?
            .queue(Print(self.border_chars.tl))?;

        for _ in pos.x+1..pos.x+size.width-1 {
            stdout()
                .queue(Print(self.border_chars.top))?;
        }

        stdout()
            .queue(Print(self.border_chars.tr))?;

        // endregion: -- Top Row

        // region:    -- Sides

        for y in pos.y+1..pos.y+size.height-1 {
            stdout()
                .queue(MoveTo(pos.x, y))?
                .queue(Print(self.border_chars.left))?
                .queue(MoveTo(pos.x+size.width-1, y))?
                .queue(Print(self.border_chars.right))?;
        }

        // endregion: -- Sides

        // region:    -- Bottom Row

        stdout()
            .queue(MoveTo(pos.x, pos.y+size.height-1))?
            .queue(Print(self.border_chars.bl))?;

        for _ in pos.x+1..pos.x+size.width-1 {
            stdout()
            .queue(Print(self.border_chars.bottom))?;
        }

        stdout()
            .queue(Print(self.border_chars.br))?;

        // endregion: -- Bottom Row
        }

        Ok(())
    }

    fn draw_split(&self) -> Result<()> {
        if let (Some(size), Some(pos)) = (&self.size, &self.position) {
            let y = pos.y + size.height - 3;
            
            stdout()
                .queue(MoveTo(pos.x, y))?
                .queue(Print(self.border_chars.left_intersect))?;

            for _ in pos.x+1..pos.x+size.width-1 {
                stdout()
                    .queue(Print(self.border_chars.split))?;
            }

            stdout()
                .queue(Print(self.border_chars.right_intersect))?;
        }
        Ok(())
    }

    fn draw_fields(&self) -> Result<()> {
        if let (Some(size), Some(position)) = (&self.size, &self.position) {
            self
                .fields
                .iter()
                .try_for_each(|field| self
                    .fields
                    .draw_field(field, DialogSpecs { position: position.clone(), size: size.clone(), margin: self.margin }))?;
        }

        Ok(())
    }

    fn draw_buttons(&self) -> Result<()> {
        // only handle 1-3 buttons
        if let (Some(size), Some(pos)) = (&self.size, &self.position) {
            let y = pos.y + size.height - 2;
            match (self.buttons.0.len()) {
                1 => {
                    let name = format!("<  {}  >", self.buttons.0[0].name);
                    let x = pos.x + size.width / 2 - name.len() as u16 / 2;

                    stdout()
                        .queue(MoveTo(x, y))?
                        .queue(Print(name))?;
                }

                2 => {
                    let name1 = format!("<  {}  >", self.buttons.0[0].name);
                    let name2 = format!("<  {}  >", self.buttons.0[1].name);

                    let x1 = pos.x + 2;
                    let x2 = pos.x+size.width - name2.len() as u16 - 2;

                    stdout()
                        .queue(MoveTo(x1, y))?
                        .queue(Print(name1))?
                        .queue(MoveTo(x2, y))?
                        .queue(Print(name2))?;
                }

                3 => {

                    let name1 = format!("<  {}  >", self.buttons.0[0].name);
                    let name2 = format!("<  {}  >", self.buttons.0[1].name);
                    let name3 = format!("<  {}  >", self.buttons.0[2].name);

                    let x1 = pos.x + 2;
                    let x2 = pos.x + size.width / 2 - name2.len() as u16 / 2;
                    let x3 = pos.x+size.width - name3.len() as u16 - 2;

                    stdout()
                        .queue(MoveTo(x1, y))?
                        .queue(Print(name1))?
                        .queue(MoveTo(x2, y))?
                        .queue(Print(name2))?
                        .queue(MoveTo(x3, y))?
                        .queue(Print(name3))?;

                }

                _ => panic!("Unsupported number of buttons")
            
            }
        }

        Ok(())
    }

    pub fn draw(&self) -> Result<()> {
        self.draw_border()?;
        self.draw_split()?;
        self.draw_fields()?;
        self.draw_buttons()?;
    
        stdout().flush()?;
        // calculate dialog position
        Ok(())
    }


    fn calc_size(&mut self) {
        self.size = Some(
            (
                (self.buttons.get_min_width() + 2 * self.margin + 2).max(self.fields.max_name_len+self.fields.max_display_len+self.margin*2+4),
                self.fields.count() as u16 * 2 + self.margin * 2 + 4
            ).into()
        );
    }

    fn calc_screen_size(&mut self) -> Result<()> {
        self.screen_size = Some(size()?.into());

        Ok(())
    }

    pub fn calc_pos(&mut self) {
        if let (Some(size), Some(screen_size)) = (&self.size, &self.screen_size) {
            self.position = Some((screen_size.width / 2 - size.width / 2, screen_size.height / 2 - size.height / 2).into())
        }
    }
}

#[derive(Default, Debug)]
pub struct DialogBuilder {
    title: String,
    fields: Fields,
    borders: Borders,
    buttons: Buttons,
    margin: u16,
    colors: Colors
}



impl DialogBuilder {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }
    pub fn set_borders(mut self, borders: Borders) -> Self {
        self.borders = borders;
        self
    }

    pub fn set_fields(mut self, fields: Fields) -> Self {
        self.fields = fields;

        self
    }

    pub fn set_buttons(mut self, buttons: Buttons) -> Self {
        self.buttons = buttons;

        self
    }

    pub fn set_margin(mut self, margin: u16) -> Self {
        self.margin = margin;

        self
    }

    pub fn set_colors(mut self, colors: Colors) -> Self {
        self.colors = colors;

        self
    }

    pub fn build(self) -> Dialog {
        Dialog {
            title: self.title,
            border_chars: BorderChars::new(self.borders),
            fields: self.fields,
            buttons: self.buttons,
            margin: self.margin,
            ..Default::default()
        }
        
    }
}