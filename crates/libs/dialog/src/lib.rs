use std::collections::HashMap;
use std::io::{stdout, Write};

use controls::Control;
use crossterm::cursor::SetCursorStyle;
use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::style::{Color, Colors, SetColors};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor::MoveTo, style::Print, terminal::size, QueueableCommand};
use tracing::info;
use crate::controls::UIElement;
use crate::error::Result;

use crate::{utils::{Position, Size}, borders::{BorderChars, Borders}};

pub mod borders;
pub mod controls;
mod utils;
pub mod error;
pub mod line_buffer;

#[derive(Debug, Default, Clone)]
pub enum ButtonCount {
    #[default]
    One,
    Two,
    Three
}


#[derive(Debug)]
pub struct DialogSpecs {
    pub position: Position,
    pub size: Size,
    pub margin: Position,
    pub max_name_len: usize,
    pub button_count: Option<ButtonCount>
}

#[derive(Debug)]
pub struct Dialog {
    pub position: Option<Position>,
    pub size: Option<Size>,

    pub screen_size: Option<Size>,

    title: String,
    controls: Vec<Control>,
    colors: Colors,
    overlay: Option<Color>,
    fill: bool,
    border_chars: BorderChars,
    margin: Position,
    pub is_visible: bool,
    min_width: usize,
    min_height: usize,
    focused: usize,
    button_count: Option<ButtonCount>,

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
            controls: Vec::new(),
            colors: Colors::new(Color::Black, Color::White),
            overlay: None,
            fill: true,
            border_chars: BorderChars::default(),
            margin: Position::default(),
            min_height: 4,
            min_width: 2,
            button_count: None,
            submit_result: DialogResult::Ok,
            cancel_result: DialogResult::Cancel,
            is_visible: false,
            focused: 0
        }
    }
}


impl Dialog {

    pub fn get_data(&self) -> FormData {
        FormData::new(self.controls.clone().into_iter().filter_map(|control| {
            control.get_value()
        }).collect::<HashMap<String, String>>())
    }

    pub fn max_name_len(&self) -> usize {
        self.controls.iter().map(|c| if let Control::TextField(field) = c {
            field.get_name().len()
        } else {
            0
        }).max().unwrap_or(0)
    }

    pub fn resize(&mut self) -> Result<()> {
        self.calc_size()?;
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
            Some(DialogSpecs { position: position.clone(), size: size.clone(), margin: self.margin.clone(), max_name_len: self.max_name_len(), button_count: self.button_count.clone() })
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
           .queue(MoveTo(pos.x as u16, pos.y as u16))?
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
                .queue(MoveTo(pos.x as u16, y as u16))?
                .queue(Print(self.border_chars.left))?
                .queue(MoveTo((pos.x+size.width-1) as u16, y as u16))?
                .queue(Print(self.border_chars.right))?;
        }

        // endregion: -- Sides

        // region:    -- Bottom Row

        stdout()
            .queue(MoveTo(pos.x as u16, (pos.y+size.height-1) as u16))?
            .queue(Print(self.border_chars.bl))?;

        for _ in pos.x+1..pos.x+size.width-1 {
            stdout()
            .queue(Print(self.border_chars.bottom))?;
        }

        stdout()
            .queue(Print(self.border_chars.br))?;

        // endregion: -- Bottom Row

        // region:    -- Fill
        let clear = " ".repeat(size.width - 2);
        if self.fill {
            for y in pos.y+1..pos.y+size.height-1 {
                stdout()
                    .queue(MoveTo((pos.x+1) as u16, y as u16))?
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
                .queue(MoveTo(pos.x as u16, y as u16))?
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

    fn draw_controls(&self) -> Result<()> {
        if let (Some(_), Some(_)) = (&self.size, &self.position) {
            self
                .controls
                .iter()
                .try_for_each(|control| control.draw())?;
        }

        Ok(())
    }

    fn draw_title(&self) -> Result<()> {
        
        if let (Some(_size), Some(pos)) = (&self.size, &self.position) {            
            stdout()
                .queue(MoveTo((pos.x+4) as u16, pos.y as u16))?
                .queue(Print(&self.title))?;
        }

        Ok(())
    }

    fn get_focused_control(&mut self) -> Option<&mut Control> {
        self
            .controls
            .iter_mut()
            .filter(|c| c.get_tab_index().is_some())
            .find(|c| c.get_tab_index() == Some(self.focused))
    }

    pub fn set_focus(&mut self) -> Result<()> {
        let mut control = self.get_focused_control();
        
        if control.is_none() {
            self.focused = 0;
            control = self.get_focused_control();
        }
         
        if let Some(control) = control {
            control.show_focus_indicator()?;
            stdout().flush()?;
        }

        Ok(())
    }
    
    pub fn focus_last(&mut self) -> Result<()> {        
        self
            .controls
            .iter_mut()
            .filter_map(|control| control
                .get_tab_index()
                .map(|tab_index| (control, tab_index)))
            .max_by_key(|(_, tab_index)| *tab_index)
            .map(|(control, _)| {
                self.focused = control.get_tab_index().unwrap_or(0);
                control.show_focus_indicator()
            }).transpose()?;

        Ok(())
    }

    fn defocus(&mut self) -> Result<()> {
        if let Some(control) = self.get_focused_control() {
            control.hide_focus_indicator()?;
            stdout().flush()?;
        }

        Ok(())
    }

    pub fn focus_next(&mut self) -> Result<()> {
        self.defocus()?;

        self.focused = self.focused.saturating_add(1);
        let mut control = self.get_focused_control();

        if control.is_none() {
            self.focused = 0;
            control = self.get_focused_control();            
        }

        if let Some(control) = control {
            control.show_focus_indicator()?;
            stdout().flush()?;
        }

        Ok(())
    }

    pub fn focus_previous(&mut self) -> Result<()> {
        self.defocus()?;

        if self.focused == 0 {
            self.focus_last()?;
            stdout().flush()?;
            return Ok(())
        }

        self.focused -= 1;

        if let Some(control) = self.get_focused_control() {
            control.show_focus_indicator()?;
            stdout().flush()?;
        }


        Ok(())
    }

    

    pub fn handle_input(&mut self, code: KeyCode, modifiers: KeyModifiers) -> Result<DialogReturnValue> {
        match (code, modifiers) {
            (KeyCode::Enter, _) => {
                return Ok(DialogReturnValue { should_quit: true, dialog_result: Some(self.submit_result.clone()) });
            }
            (KeyCode::Esc, _) => {
                return Ok(DialogReturnValue { should_quit: true, dialog_result: Some(self.cancel_result.clone()) });
            }
            (KeyCode::Tab, _) => {
                self.focus_next()?;
            }
            (KeyCode::BackTab, _) => {
                self.focus_previous()?;
            }
            _ => {}
        }

        if let Some(focusable) = self.get_focused_control().map(|c| c as &mut Control) { 
            return focusable.handle_input(code, modifiers);
        }


        Ok(DialogReturnValue::default())
    }

    pub fn draw(&mut self) -> Result<()> {
        self.draw_overlay()?;
        stdout().queue(SetColors(self.colors))?;
        self.draw_border()?;
        self.draw_title()?;
        self.draw_split()?;
        self.draw_controls()?;
        // self.draw_buttons()?;
        self.set_focus()?;
        stdout().queue(SetCursorStyle::SteadyBar)?;
    
        stdout().flush()?;
        // calculate dialog position
        Ok(())
    }

    fn calc_size(&mut self) ->Result<()> {
        self.size = Some((self.min_width, self.min_height).into());

        Ok(())
    }

    fn calc_screen_size(&mut self) -> Result<()> {
        self.screen_size = Some(size()?.into());

        Ok(())
    }

    pub fn calc_pos(&mut self) {
        if let (Some(size), Some(screen_size)) = (&self.size, &self.screen_size) {
            self.position = Some((screen_size.width / 2 - size.width / 2, screen_size.height / 2 - size.height / 2).into());
            self.set_control_positions();
        }
    }

    fn set_control_positions(&mut self) {
        let specs = self.get_dialog_specs();
        if let Some(specs) = specs {
            self.controls.iter_mut().for_each(|c| {
                match c {
                    Control::TextField(field) => {
                        let x = specs.position.x + 1 + specs.margin.x + specs.max_name_len - field.get_name().len();
                        let y = specs.position.y + 1 + specs.margin.y + 2*field.get_field_index();
                        field.set_position((x, y).into());
                    },
                    Control::Button(button) => {
                        if let Some(button_count) = &self.button_count {
                            let y = specs.position.y + specs.size.height - 2;
                            let x = match (&button.index, button_count) {
                                (_, ButtonCount::One) | (ButtonCount::Two, ButtonCount::Three) => specs.position.x + specs.size.width/2 - button.name.len(),
                                (ButtonCount::One, ButtonCount::Two) | (ButtonCount::One, ButtonCount::Three) => specs.position.x + specs.margin.x+1,
                                (ButtonCount::Two, ButtonCount::Two) | (ButtonCount::Three, ButtonCount::Three) => specs.position.x+specs.size.width-specs.margin.x-button.name.len() - 1,    
                                _ => panic!("Invalid button specs")

                            };
                            button.set_position((x, y).into());
                        }
                    },
                }
            });
        }
    }
}

#[derive(Debug)]
pub struct DialogBuilder {
    title: String,
    controls: Vec<Control>,
    borders: Borders,
    margin: Position,
    colors: Colors,
    overlay: Option<Color>,
    fill: bool,
    min_width: usize,
    min_height: usize,
    submit_result: DialogResult,
    cancel_result: DialogResult,
    button_count: Option<ButtonCount>
}

impl Default for DialogBuilder {
    fn default() -> Self {
        Self {
            title: String::default(),
            controls: Vec::new(),
            borders: Borders::default(),
            margin: Position::default(),
            colors: Colors::new(Color::White, Color::Black),
            overlay: None,
            fill: true,
            min_width: 2,
            min_height: 4,
            submit_result: DialogResult::Ok,
            cancel_result: DialogResult::Cancel,
            button_count: None
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

    pub fn add_control(mut self, control: Control) -> Self {

        let new_dimensions = match &control {
            Control::TextField(field) => {
                (self.min_width.max(4 + field.get_display_window() + field.get_name().len() + 2 * self.margin.x), self.min_height + 2)
            },
            Control::Button(button) => {
                self.button_count = match self.button_count {
                    None => Some(ButtonCount::One),
                    Some(ButtonCount::One) => Some(ButtonCount::Two),
                    Some(ButtonCount::Two) => Some(ButtonCount::Three),
                    Some(ButtonCount::Three) => panic!("You can only add 3 buttons.")
                };
                (self.min_width.max(button.name.len() + 2), self.min_height)
            },
        };

        self.min_width = new_dimensions.0;
        self.min_height = new_dimensions.1;


        self.controls.push(control);

        self
    }

    pub fn set_margin(mut self, margin: Position) -> Self {        
        self.min_width += margin.x * 2;
        self.min_height += margin.y * 2;

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
        let mut controls = self.controls.clone();
        controls.sort_by_key(|c| c.get_tab_index());
        
        Dialog {
            title: self.title,
            border_chars: BorderChars::new(self.borders),
            controls,
            margin: self.margin,
            colors: self.colors,
            overlay: self.overlay,
            fill: self.fill,
            submit_result: self.submit_result,
            cancel_result: self.cancel_result,
            min_height: self.min_height,
            min_width: self.min_width,
            button_count: self.button_count,
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