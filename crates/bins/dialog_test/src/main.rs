use std::io::{stdout, Write};
use std::time::Duration;
use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers};
use crossterm::style::{Color, Colors, SetColors};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::QueueableCommand;

use dialog::borders::{BorderStyle, Borders};
use dialog::controls::button::{Button, Buttons};
use dialog::controls::field::{Field, Fields};
use dialog::error::Result;
use dialog::{DialogBuilder, DialogReturnValue};

#[derive(PartialEq)]
enum Mode {
    Insert,
    _Overtype
}

fn main() -> Result<()> {
    // region:    -- Setup Dialog

    let buttons = Buttons::new(vec![
        Button::new("OK", Some(0), dialog::DialogResult::Ok),
        Button::new("Cancel", Some(1), dialog::DialogResult::Cancel)
    ]);

    let mut fields = Fields::new();

    fields.add_field(Field::new("First Name", 15, 15, Some(0)));
    fields.add_field(Field::new("Last Name", 15, 15, Some(1)));
    fields.add_field(Field::new("Company Name", 15, 40, Some(2)));
    fields.add_field(Field::new("Phone Number", 15, 15, Some(3)));
    
    

    let borders = Borders {
        top: BorderStyle::Double,
        left: BorderStyle::Single,
        right: BorderStyle::Single,
        bottom: BorderStyle::Double,
        split: BorderStyle::Double
    };

    let mut dialog = DialogBuilder::new(" Add Contact ")
        .set_borders(borders)
        .set_margin((4, 1).into())
        .set_buttons(buttons)
        .set_fields(fields)
        .set_colors(Colors::new(Color::Black, Color::White))
        .build();
    
    dialog.resize()?;
    

    // endregion: -- Setup Dialog

    // region:    -- Setup RawMode
    stdout()
        .queue(EnterAlternateScreen)?
        .flush()?;
    
    enable_raw_mode()?;
    // endregion: -- Setup RawMode

    
    draw_background()?;
    let mut _mode = Mode::Insert;
    let mut result = DialogReturnValue::default();
    loop {
        if poll(Duration::from_secs(0))? {
            if let Event::Key(event) = read()? {
                match (event.code, event.modifiers) {
                    (KeyCode::Char(' '), _) => {
                        if dialog.is_visible {
                            dialog.hide()?;
                            draw_background()?;
                        } else {
                            dialog.show()?;
                        }
                    },
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => break,
                    (code, modifiers) => {
                        result = dialog.handle_input(code, modifiers);
                        
                        if result.should_quit {
                            break;
                        }
                    }
                }
            }
        }
    }

    // region:    -- End RawMode
    disable_raw_mode()?;
    stdout().queue(LeaveAlternateScreen)?.flush()?;
    // endregion: -- End RawMode
    
    println!("Exited with: {:?}", result);
    Ok(())
}

fn draw_background() ->Result<()> {
    let colors = Colors::new(Color::Blue, Color::DarkBlue);
    stdout()
        .queue(SetColors(colors))?
        .queue(Clear(ClearType::All))?.flush()?;

    Ok(())
}
