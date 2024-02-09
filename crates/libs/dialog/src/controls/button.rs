// region:    -- Buttons

use std::ops::Index;

use crate::DialogResult;

#[derive(Debug, Default, Clone)]
pub struct Button {
    pub name: String,
    pub tab_index: Option<u8>,
    pub result: DialogResult,
}

impl Button {
    pub fn new(name: impl Into<String>, tab_index: Option<u8>, result: DialogResult) -> Self {
        Self {
            name: name.into(),
            tab_index,
            result
         }
    }
}

#[derive(Debug)]
pub struct Buttons {
    pub buttons: Vec<Button>
}

impl Buttons {
    pub fn get_min_width(&self) -> u16 {
        self.buttons.iter().map(|b| b.name.len() as u16+6).sum()
    }

    pub fn new(buttons: Vec<Button>) -> Self {
        Self {
            buttons
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Button> {
        self.buttons.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Button> {
        self.buttons.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.buttons.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buttons.is_empty()
    }
}

impl Index<usize> for Buttons {
    type Output = Button;

    fn index(&self, index: usize) -> &Self::Output {
        &self.buttons[index]
    }
}

// Implement IntoIterator for Buttons
impl IntoIterator for Buttons {
    type Item = Button;
    type IntoIter = std::vec::IntoIter<Button>;

    fn into_iter(self) -> Self::IntoIter {
        self.buttons.into_iter()
    }
}

impl Default for Buttons {
    fn default() -> Self {
        let buttons = vec![
            Button::new("OK", Some(0), DialogResult::Ok),
            Button::new("Cancel", Some(1), DialogResult::Cancel)
        ];

        Self { buttons }
    }
}

// endregion: -- Buttons