// region:    -- Fields
use crate::{error::Result, line_buffer::{LineBuffer, TextMode}, utils::Position};

use std::io::stdout;

use crossterm::{cursor::MoveTo, event::{KeyCode, KeyModifiers}, style::Print, QueueableCommand};

#[derive(Debug, Default, Clone)]
pub struct Field {
    pub name: String,
    pub display_len: usize,
    pub input_len: usize,
    pub tab_index: Option<usize>,
    pub index: usize,
    pub value: String,
    pub position: Position,
    line_buffer: LineBuffer,
}

impl Field {
    pub fn new(name: impl Into<String>, display_len: usize, input_len: usize, tab_index: Option<usize>, index: usize) -> Self {
        Self {
            name: name.into(),
            display_len,
            input_len,
            tab_index,
            index,
            value: String::new(),
            line_buffer: LineBuffer::new(display_len, input_len, (0, 0).into(), '_', TextMode::Insert),
            position: (0, 0).into(),
        }
    }

    pub fn handle_text_input(&mut self, code: KeyCode, modifiers: KeyModifiers) -> Result<()> {
        self.line_buffer.handle_input(code, modifiers)?;
        self.value = self.line_buffer.buffer.clone();

        Ok(())
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position.clone();
        let pos = Position { x: position.x+self.name.len()+2, y: position.y };
        self.line_buffer.set_position(pos);
    }

    fn draw_label(&self) -> Result<()> {
        stdout()
            .queue(MoveTo(self.position.x as u16, self.position.y as u16))?
            .queue(Print(format!("{}: ", self.name)))?;

        Ok(())
    }

    pub fn draw(&self) -> Result<()> {
        self.draw_label()?;
        self.line_buffer.draw()?;

        Ok(())
    }


}

// #[derive(Debug, Default)]
// pub struct Fields{
//     fields: Vec<Field>,
//     pub max_name_len: usize,
//     pub max_display_len: usize
// }

// impl Fields {
//     pub fn iter(&self) -> std::slice::Iter<'_, Field> {
//         self.fields.iter()
//     }
//     pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Field> {
//         self.fields.iter_mut()
//     }
    
//     pub fn new() -> Self {
//         Default::default()
//     }

//     pub fn add_field(&mut self, mut field: Field) {
//         field.index = self.fields.len();
        
//         self.max_name_len = self.max_name_len.max(field.name.len());
//         self.max_display_len = self.max_display_len.max(field.display_len);
        
//         self.fields.push(field);
//     }

//     pub fn len(&self) -> usize {
//         self.fields.len()
//     }

//     pub fn is_empty(&self) -> bool {
//         self.fields.is_empty()
//     }
// }

// impl Index<usize> for Fields {
//     type Output = Field;

//     fn index(&self, index: usize) -> &Self::Output {
//         &self.fields[index]
//     }
// }

// // Implement IntoIterator for Fields
// impl IntoIterator for Fields {
//     type Item = Field;
//     type IntoIter = std::vec::IntoIter<Field>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.fields.into_iter()
//     }
// }

// // endregion: -- Fields
