use std::io::{stdout, Write};

use crossterm::cursor::{MoveTo, SetCursorStyle};
use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{ExecutableCommand, QueueableCommand};
use tracing::{info, instrument};

use crate::utils::Position;
use crate::error::Result;

#[derive(Debug, Default, Clone)]
pub enum TextMode {
    Overtype,
    #[default]
    Insert
}

#[derive(Debug, Default, Clone)]
pub struct LineBuffer {
    window_size: usize,
    max_buffer_len: usize,
    position: Position,
    pad_char: char,
    pub buffer: String,
    pos: usize,
    window_start: usize,
    mode: TextMode
}

impl LineBuffer {
    #[instrument]
    pub fn new(window_size: usize, max_buffer_len: usize, position: Position, pad_char: char, mode: TextMode) -> Self {
        Self {
            window_size,
            max_buffer_len,
            position,
            pad_char,
            buffer: String::new(),
            pos: 0,
            window_start: 0,
            mode
        }
    }

    #[instrument]
    pub fn handle_input(&mut self, code: KeyCode, _modifiers: KeyModifiers) -> Result<()> {
        match code {
            KeyCode::Left => {
                self.move_left();
            }
            KeyCode::Right => {
                self.move_right();
            }
            KeyCode::Home => {
                self.move_home();
            }
            KeyCode::End => {
                self.move_end();
            }
            KeyCode::Backspace => {
                self.backspace();
            }
            KeyCode::Delete => {
                self.delete();
            }
            KeyCode::Char(char) => {
                self.add_char(char);
                
            }
            _ => {}
        }
        self.draw()?;

        Ok(())
    }

    #[instrument]
    pub fn toggle_insert(&mut self) -> Result<()> {
        self.mode = match self.mode {
            TextMode::Overtype => TextMode::Insert,
            TextMode::Insert => TextMode::Overtype,
        };
        
        stdout().execute(self.get_cursor_style())?;
        
        Ok(())
    }

    #[instrument]
    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }
    
    #[instrument]
    pub fn get_position(&self) -> Position {
        self.position.clone()
    }
    
    #[instrument]
    pub fn set_pos(&mut self, pos: usize) {
        self.pos = pos.min(self.buffer.len());
        self.adjust_visible_window();
    }

    #[instrument]
    fn adjust_visible_window(&mut self) {
        let window_size = self.window_size.min(self.buffer.len());
    
        if self.pos < self.window_start {
            self.window_start = self.pos;
        } 
        // Change here: use '>' instead of '>=' to delay the window shift until the cursor is truly beyond the visible window
        else if self.pos > self.window_start + window_size {
            self.window_start = self.pos - window_size;
        }
    }

    #[instrument]
    fn add_char(&mut self, c: char) -> CharAddResult {
        info!("add_char: {c}");
        match self.mode {
            TextMode::Overtype => {
                // Correct the boundary check to prevent additions beyond the buffer's maximum length
                if self.buffer.len() >= self.max_buffer_len && self.pos >= self.max_buffer_len {
                    return CharAddResult::Rejected;
                }
    
                if self.pos == self.buffer.len() {
                    // Ensure we only add new characters if within the maximum buffer length
                    if self.buffer.len() < self.max_buffer_len {
                        self.buffer.push(c);
                    }
                } else {
                    // Replace character at self.pos without changing the buffer size
                    self.buffer = self
                        .buffer
                        .char_indices()
                        .map(|(i, existing)| if i == self.pos { c } else { existing })
                        .collect();
                }
            },
            TextMode::Insert => {
                // Prevent insertions beyond the maximum buffer length
                if self.buffer.len() >= self.max_buffer_len {
                    return CharAddResult::Rejected;
                }
    
                if self.pos == self.buffer.len() {
                    self.buffer.push(c);
                } else {
                    let (before, after) = self
                        .buffer
                        .char_indices()
                        .partition::<Vec<_>, _>(|(i, _)| i < &self.pos);
                    
                    self.buffer = before
                        .into_iter()
                        .chain(std::iter::once((self.pos, c)))
                        .chain(after)
                        .map(|(_, c)| c)
                        .collect();
                }
            },
        };
    
        // Move the cursor position forward
        self.set_pos(self.pos+1);
        CharAddResult::Accepted
    }
    
    #[instrument]
    fn backspace(&mut self) {
        if self.pos > 0 {
            self.buffer.remove(self.pos - 1);
            self.set_pos(self.pos-1);
        }
    }

    #[instrument]
    fn delete(&mut self) {
        if self.pos < self.buffer.len() {
            self.buffer.remove(self.pos);
            self.adjust_visible_window();
        }
    }
    
    #[instrument]
    fn move_left(&mut self) {
        if self.pos > 0 {
            self.set_pos(self.pos-1);
        }
    }

    #[instrument]
    fn move_right(&mut self) {
        if self.pos < self.buffer.len() {
            self.set_pos(self.pos+1);
        }
    }

    #[instrument]
    fn move_home(&mut self) {
        self.set_pos(0);
    }

    #[instrument]
    fn move_end(&mut self) {
        self.set_pos(self.buffer.len());
    }

    #[instrument]
    fn get_cursor_style(&self) -> SetCursorStyle {
        match self.mode {
            TextMode::Overtype => SetCursorStyle::SteadyUnderScore,
            TextMode::Insert => SetCursorStyle::SteadyBar,
        }
    }

    #[instrument]
    pub fn draw(&self) -> Result<()> {
        let window_end = std::cmp::min(self.window_start + self.window_size, self.buffer.len());
    
        let has_left_text = self.window_start > 0;
        let has_right_text = window_end < self.buffer.len();
    
        let visible_buffer = &self.buffer[self.window_start..std::cmp::min(window_end, self.window_start + self.window_size)];
    
        let cursor_pos_within_window = self.pos.saturating_sub(self.window_start);

        stdout().queue(MoveTo((self.position.x - 1) as u16, self.position.y as u16))?;
    
        if has_left_text { stdout().queue(Print("<"))?; } else { stdout().queue(Print(" "))?; }
        
        stdout().queue(Print(visible_buffer))?;
        
        if has_right_text { stdout().queue(Print(">"))?; } else { stdout().queue(Print(" "))?; }
    
        let pad_length = self.window_size.saturating_sub(visible_buffer.chars().count());
        stdout().queue(Print(self.pad_char.to_string().repeat(pad_length)))?;
    
        stdout()
            .queue(MoveTo(self.position.x as u16 + cursor_pos_within_window as u16, self.position.y as u16))?
            .queue(self.get_cursor_style())?
            .flush()?;
    
        Ok(())
    }
    
    
}
enum CharAddResult {
    Accepted,
    Rejected
}