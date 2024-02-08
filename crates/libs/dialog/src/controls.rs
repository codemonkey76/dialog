use std::io::{stdout, Write};

use crossterm::{cursor::MoveTo, style::Print, QueueableCommand};

use crate::{error::Result, utils::{Position, Size}, DialogSpecs};

// region:    -- Fields

#[derive(Debug, Default)]
pub struct Field {
    pub name: String,
    pub display_len: u16,
    pub input_len: u16,
    pub tab_index: Option<u8>,
    pub index: u16
}

impl Field {
    pub fn new(name: impl Into<String>, display_len: u16, input_len: u16, tab_index: Option<u8>) -> Self {
        Self {
            name: name.into(),
            display_len,
            input_len,
            tab_index,
            index: 0
        }
    }
}

#[derive(Debug, Default)]
pub struct Fields{
    fields: Vec<Field>,
    pub max_name_len: u16,
    pub max_display_len: u16
}

impl Fields {
    pub fn iter(&self) -> std::slice::Iter<'_, Field> {
        self.fields.iter()
    }
    
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_field(&mut self, mut field: Field) {
        field.index = self.fields.len() as u16;
        
        self.max_name_len = self.max_name_len.max(field.name.len() as u16);
        self.max_display_len = self.max_display_len.max(field.display_len);
        
        self.fields.push(field);
    }

    pub fn draw_field(&self, field: &Field, specs: DialogSpecs) -> Result<()> {
        let x = specs.position.x + 1 + specs.margin + self.max_name_len - field.name.len() as u16;
        let y = specs.position.y + 1 + specs.margin + 2*field.index;
        let pad = "_".repeat(field.display_len as usize);
        
        stdout()
            .queue(MoveTo(x, y))?
            .queue(Print(format!("{}: ", &field.name)))?
            .queue(Print(pad))?
            .flush()?;
        Ok(())

    }

    pub fn count(&self) -> usize {
        self.fields.len()
    }
}

// Implement IntoIterator for Fields
impl IntoIterator for Fields {
    type Item = Field;
    type IntoIter = std::vec::IntoIter<Field>;

    fn into_iter(self) -> Self::IntoIter {
        self.fields.into_iter()
    }
}

// endregion: -- Fields


// region:    -- Buttons

#[derive(Debug, Default)]
pub struct Button {
    pub name: String
}
impl Button {
    pub fn new(name: impl Into<String>, tab_index: Option<u8>) -> Self {
        Self { name: name.into() }
    }
}

#[derive(Debug)]
pub struct Buttons(pub Vec<Button>);

impl Buttons {
    pub fn get_min_width(&self) -> u16 {
        self.0.iter().map(|b| b.name.len() as u16+6).sum()
    }
}

impl Default for Buttons {
    fn default() -> Self {
        let buttons = vec![
            Button::new("OK", Some(0)),
            Button::new("Cancel", Some(1))
        ];

        Self(buttons)
    }
}

// endregion: -- Buttons