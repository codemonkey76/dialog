use std::io::{stdout, Write};

use crossterm::{cursor::MoveTo, style::Print, terminal::size, QueueableCommand};

use crate::error::Result;

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

#[derive(Debug)]
struct Position {
    x: u16,
    y: u16
}

impl From<(u16, u16)> for Position {
    fn from(value: (u16, u16)) -> Self {
        Self { x: value.0, y: value.1 }
    }
}

#[derive(Debug)]
struct Size {
    width: u16,
    height: u16
}

impl From<(u16, u16)> for Size {
    fn from(value: (u16, u16)) -> Self {
        Self { width: value.0, height: value.1 }
    }
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

    pub fn draw(&self) -> Result<()> {
        self.draw_border()?;
        self.draw_split()?;
    
        stdout().flush()?;
        // calculate dialog position
        Ok(())
    }


    fn calc_size(&mut self) {
        self.size = Some(
            (
                self.buttons.get_min_width() + 2 * self.margin + 2,
                self.fields.count() * 2 + self.margin * 2 + 4
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
    margin: u16
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


#[derive(Debug, Default)]
pub struct Field {


}

#[derive(Debug, Default)]
pub struct Fields(Vec<Field>);

impl Fields {
    pub fn count(&self) -> u16 {
        self.0.len() as u16
    }
}



#[derive(Debug, Default)]
pub struct Borders {
    top: BorderStyle,
    left: BorderStyle,
    right: BorderStyle,
    bottom: BorderStyle,
    split: BorderStyle
}

#[derive(Debug, Default)]
pub enum BorderStyle {
    Single,
    #[default]
    Double
}

#[derive(Debug, Default)]
pub struct Button {
    name: String
}
impl Button {
    pub fn new(name: impl Into<String>, tab_index: Option<u8>) -> Self {
        Self { name: name.into() }
    }
}

#[derive(Debug)]
pub struct Buttons(Vec<Button>);

impl Buttons {
    fn get_min_width(&self) -> u16 {
        self.0.iter().map(|b| b.name.len() as u16).sum()
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


#[derive(Debug)]
struct BorderChars {
    tl: char,
    tr: char,
    bl: char,
    br: char,

    top: char,
    left: char,
    right: char,
    bottom: char,

    left_intersect: char,
    right_intersect: char,
    split: char
}

impl Default for BorderChars {
    fn default() -> Self {
        Self::new(Borders::default())
    }
}

impl BorderChars {
    fn new(borders: Borders) -> Self {
        /*
        
                0	1	2	3	4	5	6	7	8	9	A	B	C	D	E	F
            B	░	▒	▓	│	┤	╡	╢	╖	╕	╣	║	╗	╝	╜	╛	┐
            C	└	┴	┬	├	─	┼	╞	╟	╚	╔	╩	╦	╠	═	╬	╧
            D	╨	╤	╥	╙	╘	╒	╓	╫	╪	┘	┌	█	▄	▌	▐	▀
        
         */

        let left_intersect = match (&borders.left, &borders.split) {
            (BorderStyle::Single, BorderStyle::Single) => '├',
            (BorderStyle::Single, BorderStyle::Double) => '╞',
            (BorderStyle::Double, BorderStyle::Single) => '╟',
            (BorderStyle::Double, BorderStyle::Double) => '╠',
        };

        let right_intersect = match (&borders.right, &borders.split) {
            (BorderStyle::Single, BorderStyle::Single) => '┤',
            (BorderStyle::Single, BorderStyle::Double) => '╡',
            (BorderStyle::Double, BorderStyle::Single) => '╢',
            (BorderStyle::Double, BorderStyle::Double) => '╣',
        };

        let tl = match (&borders.top, &borders.left) {
            (BorderStyle::Single, BorderStyle::Single) => '┌',
            (BorderStyle::Single, BorderStyle::Double) => '╓',
            (BorderStyle::Double, BorderStyle::Single) => '╒',
            (BorderStyle::Double, BorderStyle::Double) => '╔',
        };

        let tr = match(&borders.top, &borders.right) {
            (BorderStyle::Single, BorderStyle::Single) => '┐',
            (BorderStyle::Single, BorderStyle::Double) => '╖',
            (BorderStyle::Double, BorderStyle::Single) => '╕',
            (BorderStyle::Double, BorderStyle::Double) => '╗',
        };

        let bl = match(&borders.bottom, &borders.left) {
            (BorderStyle::Single, BorderStyle::Single) => '└',
            (BorderStyle::Single, BorderStyle::Double) => '╙',
            (BorderStyle::Double, BorderStyle::Single) => '╘',
            (BorderStyle::Double, BorderStyle::Double) => '╚',
        };

        let br = match(&borders.bottom, &borders.right) {
            (BorderStyle::Single, BorderStyle::Single) => '┘',
            (BorderStyle::Single, BorderStyle::Double) => '╜',
            (BorderStyle::Double, BorderStyle::Single) => '╛',
            (BorderStyle::Double, BorderStyle::Double) => '╝',
        };

        let top = match &borders.top {
            BorderStyle::Single => '─',
            BorderStyle::Double => '═',
        };

        let left = match &borders.left {
            BorderStyle::Single => '│',
            BorderStyle::Double => '║',
        };

        let right = match &borders.right {
            BorderStyle::Single => '│',
            BorderStyle::Double => '║',
        };

        let bottom = match &borders.bottom {
            BorderStyle::Single => '─',
            BorderStyle::Double => '═',
        };

        let split = match &borders.split {
            BorderStyle::Single => '─',
            BorderStyle::Double => '═',
        };

        Self { tl, tr, bl, br, top, left, right, bottom, left_intersect, right_intersect, split }

    }
}