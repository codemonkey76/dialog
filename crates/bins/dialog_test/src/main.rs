use std::io::{stdout, Write};
use std::time::Duration;
use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::QueueableCommand;

use dialog::borders::{BorderStyle, Borders};
use dialog::controls::{Button, Buttons, Field, Fields};
use dialog::error::Result;
use dialog::DialogBuilder;

fn main() -> Result<()> {
    // region:    -- Setup Dialog

    let buttons = Buttons(vec![
        Button::new("OK", Some(0)),
        Button::new("Cancel", Some(1))
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

    let mut dialog = DialogBuilder::new("test")
        .set_borders(borders)
        .set_margin(2)
        .set_buttons(buttons)
        .set_fields(fields)
        .build();
    
    dialog.resize()?;
    

    // endregion: -- Setup Dialog

    // region:    -- Setup RawMode
    stdout()
        .queue(EnterAlternateScreen)?
        .flush()?;
    
    enable_raw_mode()?;
    // endregion: -- Setup RawMode

    dialog.draw()?;
    
    loop {
        if poll(Duration::from_secs(0))? {
            if let Event::Key(event) = read()? {
                match (event.code, event.modifiers) {
                    (KeyCode::Char(' '), _) => break,
                    _ => {}
                }
            }
        }
    }

    // region:    -- End RawMode
    disable_raw_mode()?;
    stdout().queue(LeaveAlternateScreen)?.flush()?;
    // endregion: -- End RawMode
    
    Ok(())
}
