use std::collections::HashMap;
use std::io::{stdout, Write};

use controls::{Control, Focusable};
use crossterm::cursor::SetCursorStyle;
use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::style::{Color, Colors, SetColors};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor::MoveTo, style::Print, terminal::size, QueueableCommand};

use crate::error::Result;

use crate::{utils::{Position, Size}, borders::{BorderChars, Borders}};

pub mod borders;
pub mod controls;
mod utils;
pub mod error;


#[derive(Debug)]
pub struct DialogSpecs {
    pub position: Position,
    pub size: Size,
    pub margin: Position,
    pub max_name_len: usize
}

#[derive(Debug)]
pub struct Dialog {
    pub position: Option<Position>,
    pub size: Option<Size>,

    pub screen_size: Option<Size>,

    title: String,
    fields: controls::field::Fields,
    colors: Colors,
    overlay: Option<Color>,
    fill: bool,
    buttons: controls::button::Buttons,
    border_chars: BorderChars,
    margin: Position,
    pub is_visible: bool,
    focused: u16,

    submit_result: DialogResult,
    cancel_result: DialogResult
}

impl Default for Dialog {
    fn default() -> Self {
        Self {
            position: None,
            size: None,
            screen_size: None,
            title: String::default(),
            fields: controls::field::Fields::default(),
            colors: Colors::new(Color::Black, Color::White),
            overlay: None,
            fill: true,
            buttons: controls::button::Buttons::default(),
            border_chars: BorderChars::default(),
            margin: Position::default(),
            submit_result: DialogResult::Ok,
            cancel_result: DialogResult::Cancel,
            is_visible: false,
            focused: 0
        }
    }
}



impl Dialog {

    pub fn get_data(&self) -> FormData {
        FormData::new(self.fields.iter().map(|f| (f.name.clone(), f.value.clone())).collect::<HashMap<String,String>>())
    }
    pub fn resize(&mut self) -> Result<()> {
        self.calc_size();
        self.calc_screen_size()?;
        self.calc_pos();

        Ok(())
    }
    
    pub fn show(&mut self) -> Result<()> {
        self.is_visible = true;
        
        self.draw()?;
        
        Ok(())
    }

    pub fn hide(&mut self) -> Result<()> {
        self.is_visible = false;
        self.draw()?;

        Ok(())
    }
    
    fn get_dialog_specs(&self) -> Option<DialogSpecs> {
        if let (Some(size), Some(position)) = (&self.size, &self.position) {
            Some(DialogSpecs { position: position.clone(), size: size.clone(), margin: self.margin.clone(), max_name_len: self.fields.max_name_len as usize })
        }
        else {
            None
        }
    }
    
    fn draw_overlay(&self) -> Result<()> {
        if let Some(overlay) = &self.overlay {
            stdout()
                .queue(SetColors(Colors::new(*overlay, *overlay)))?
                .queue(Clear(ClearType::All))?;
        }

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

        // region:    -- Fill
        let clear = " ".repeat(size.width as usize-2);
        if self.fill {
            for y in pos.y+1..pos.y+size.height-1 {
                stdout()
                    .queue(MoveTo(pos.x+1, y))?
                    .queue(Print(&clear))?;
            }
        }

        // endregion: -- Fill
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
        if let (Some(_), Some(_)) = (&self.size, &self.position) {
            self
                .fields
                .iter()
                .try_for_each(|field| self
                    .fields
                    .draw_field(field, self.get_dialog_specs()))?;
        }

        Ok(())
    }

    fn draw_buttons(&self) -> Result<()> {
        // only handle 1-3 buttons
        if let (Some(size), Some(pos)) = (&self.size, &self.position) {
            let y = pos.y + size.height - 2;
            match self.buttons.len() {
                1 => {
                    let name = format!("<  {}  >", self.buttons[0].name);
                    let x = pos.x + size.width / 2 - name.len() as u16 / 2;

                    stdout()
                        .queue(MoveTo(x, y))?
                        .queue(Print(name))?;
                }

                2 => {
                    let name1 = format!("<  {}  >", self.buttons[0].name);
                    let name2 = format!("<  {}  >", self.buttons[1].name);

                    let x1 = pos.x + 2;
                    let x2 = pos.x+size.width - name2.len() as u16 - 2;

                    stdout()
                        .queue(MoveTo(x1, y))?
                        .queue(Print(name1))?
                        .queue(MoveTo(x2, y))?
                        .queue(Print(name2))?;
                }

                3 => {

                    let name1 = format!("<  {}  >", self.buttons[0].name);
                    let name2 = format!("<  {}  >", self.buttons[1].name);
                    let name3 = format!("<  {}  >", self.buttons[2].name);

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

    fn draw_title(&self) -> Result<()> {
        
        if let (Some(_size), Some(pos)) = (&self.size, &self.position) {            
            stdout()
                .queue(MoveTo(pos.x+4, pos.y))?
                .queue(Print(&self.title))?;
        }

        Ok(())
    }

    fn get_focusable(&self, focus_index: u16) -> Option<&Control> {
        let mut controls = Vec::new();
        let max_focus_index = self
            .fields
            .iter()
            .filter_map(|f| {
                if let Some(ti) = f.tab_index {
                    controls.push((ti, Control::TextField(f)));
                    f.tab_index
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(0);

        self
            .buttons
            .iter_mut()
            .for_each(|b| {
                if let Some(ti) = b.tab_index {                    
                    b.tab_index = Some(ti + max_focus_index);
                    controls.push((ti+max_focus_index, Control::Button(b)));
                }
            });
        controls.sort_by_key(|(i,_)| *i);

        controls.into_iter().find(|(id, _control)| *id >= focus_index as u8).map(|(_, control)| control).as_ref()
        
    }

    pub fn set_focus(&self) -> Result<()> {    
        if let Some(focusable) = self.get_focusable(self.focused) {
            focusable.focus(self.get_dialog_specs())?;
        }

        Ok(())
    }

    pub fn handle_input(&mut self, code: KeyCode, modifiers: KeyModifiers) -> DialogReturnValue {
        match (code, modifiers) {
            (KeyCode::Enter, _) => {
                return DialogReturnValue { should_quit: true, dialog_result: Some(self.submit_result.clone()), data: self.get_data() };
            }
            (KeyCode::Esc, _) => {
                return DialogReturnValue { should_quit: true, dialog_result: Some(self.cancel_result.clone()), data: self.get_data() };
            }
            _ => {}
        }

        if let Some(focusable) = self.get_focusable(self.focused) {
            let mut control = focusable.clone();
            control.handle_input(code, modifiers);
        }


        DialogReturnValue::default()
    }

    pub fn draw(&self) -> Result<()> {
        self.draw_overlay()?;
        stdout().queue(SetColors(self.colors))?;
        self.draw_border()?;
        self.draw_title()?;
        self.draw_split()?;
        self.draw_fields()?;
        self.draw_buttons()?;
        self.set_focus()?;
        stdout().queue(SetCursorStyle::SteadyBar)?;
    
        stdout().flush()?;
        // calculate dialog position
        Ok(())
    }

    fn calc_size(&mut self) {
        self.size = Some(
            (
                (self.buttons.get_min_width() + 2 * self.margin.x + 2).max(self.fields.max_name_len+self.fields.max_display_len+self.margin.x*2+4),
                self.fields.len() as u16 * 2 + self.margin.y * 2 + 4
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

#[derive(Debug)]
pub struct DialogBuilder {
    title: String,
    fields: controls::field::Fields,
    borders: Borders,
    buttons: controls::button::Buttons,
    margin: Position,
    colors: Colors,
    overlay: Option<Color>,
    fill: bool,
    submit_result: DialogResult,
    cancel_result: DialogResult
}

impl Default for DialogBuilder {
    fn default() -> Self {
        Self {
            title: String::default(),
            fields: controls::field::Fields::default(),
            borders: Borders::default(),
            buttons: controls::button::Buttons::default(),
            margin: Position::default(),
            colors: Colors::new(Color::White, Color::Black),
            overlay: None,
            fill: true,
            submit_result: DialogResult::Ok,
            cancel_result: DialogResult::Cancel,
         }
    }
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

    pub fn set_fields(mut self, fields: controls::field::Fields) -> Self {
        self.fields = fields;

        self
    }

    pub fn set_buttons(mut self, buttons: controls::button::Buttons) -> Self {
        self.buttons = buttons;

        self
    }

    pub fn set_margin(mut self, margin: Position) -> Self {
        self.margin = margin;

        self
    }

    pub fn set_colors(mut self, colors: Colors) -> Self {
        self.colors = colors;

        self
    }

    pub fn set_fill(mut self, fill: bool) -> Self {
        self.fill = fill;

        self
    }

    pub fn set_overlay(mut self, overlay: Color) -> Self {
        self.overlay = Some(overlay);

        self
    }

    pub fn set_submit_result(mut self, result: DialogResult) -> Self {
        self.submit_result = result;

        self
    }

    pub fn set_cancel_result(mut self, result: DialogResult) -> Self {
        self.cancel_result = result;

        self
    }

    pub fn build(self) -> Dialog {
        // When building, set the button tab_indexes.

        let max_tab = self.fields.iter().map(|f| f.tab_index.unwrap_or(0)).max().unwrap_or(0);
        
        let mut buttons = self.buttons;
        buttons.iter_mut().for_each(|b| if let Some(mut tab_index) = b.tab_index {
            tab_index += max_tab;
            b.tab_index = Some(tab_index);
        });

        Dialog {
            title: self.title,
            border_chars: BorderChars::new(self.borders),
            fields: self.fields,
            buttons,
            margin: self.margin,
            colors: self.colors,
            overlay: self.overlay,
            fill: self.fill,
            submit_result: self.submit_result,
            cancel_result: self.cancel_result,
            ..Default::default()            
        }
        
    }
}

#[derive(Default, Debug, Clone)]
pub enum DialogResult {
    #[default]
    Ok,
    Cancel,
    Abort,
    Retry,
    Ignore,
    Yes,
    No,
}

#[derive(Debug, Default)]
pub struct DialogReturnValue {
    pub should_quit: bool,
    pub dialog_result: Option<DialogResult>,
    pub data: FormData
}

#[derive(Debug, Default)]
pub struct FormData {
    pub data: HashMap<String, String>
}

impl FormData {
    pub fn new(data: HashMap<String,String>) -> Self {
        Self {
            data
        }
    }
}