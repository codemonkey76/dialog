// region:    -- Fields
use crate::error::Result;

use std::{io::{stdout, Write}, ops::Index};

use crossterm::{cursor::MoveTo, event::{KeyCode, KeyModifiers}, style::Print, QueueableCommand};

#[derive(Debug, Default, Clone)]
pub struct Field {
    pub name: String,
    pub display_len: u16,
    pub input_len: u16,
    pub tab_index: Option<u8>,
    pub index: u16,
    pub value: String
}

impl Field {
    pub fn new(name: impl Into<String>, display_len: u16, input_len: u16, tab_index: Option<u8>) -> Self {
        Self {
            name: name.into(),
            display_len,
            input_len,
            tab_index,
            index: 0,
            value: String::new()
        }
    }
    pub fn handle_text_input(&mut self, code: KeyCode, modifiers: KeyModifiers) -> Result<()> {
        match (code, modifiers) {
            (KeyCode::Char(c), _) => {
                stdout().queue(MoveTo(0, 0))?.queue(Print("Handling text input"))?.flush()?;
                self.value.push(c);
            }
            _ => {}
        }

        Ok(())
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
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Field> {
        self.fields.iter_mut()
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

    pub fn draw_field(&self, field: &Field, specs: Option<crate::DialogSpecs>) -> Result<()> {
        if let Some(specs) = specs {
            let x = specs.position.x + 1 + specs.margin.x + self.max_name_len - field.name.len() as u16;
            let y = specs.position.y + 1 + specs.margin.y + 2*field.index;
            let pad = "_".repeat(field.display_len as usize);
            
            stdout()
                .queue(MoveTo(x, y))?
                .queue(Print(format!("{}: ", &field.name)))?
                .queue(Print(pad))?
                .flush()?;
        }
        Ok(())

    }

    pub fn len(&self) -> usize {
        self.fields.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }
}

impl Index<usize> for Fields {
    type Output = Field;

    fn index(&self, index: usize) -> &Self::Output {
        &self.fields[index]
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
