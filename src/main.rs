use std::io::{stdout, Write};
use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::QueueableCommand;
use dialog::{Borders, Buttons};

use crate::dialog::DialogBuilder;

mod dialog;
mod error;

use crate::error::Result;
fn main() -> Result<()> {
    let builder = DialogBuilder::new("test")
        .set_borders(Borders::default())
        .set_margin(2)
        // .set_fields()
        .set_buttons(Buttons::default());

    println!("{:?}", builder);
    let mut dialog = builder.build();
    println!("{:?}", dialog);
    dialog.resize()?;
    println!("{:?}", dialog);

    stdout()
        .queue(EnterAlternateScreen)?
        .flush()?;
    
    enable_raw_mode()?;

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


    disable_raw_mode()?;
    stdout().queue(LeaveAlternateScreen)?.flush()?;
    

    Ok(())
}
