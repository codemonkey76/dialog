use std::io::{stdout, Write};
use std::time::Duration;
use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers};
use crossterm::style::{Color, Colors};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::QueueableCommand;

use dialog::borders::{BorderStyle, Borders};
use dialog::controls::button::{Button, ButtonColors};
use dialog::controls::field::{Field, FieldColors};
use dialog::controls::Control;
use dialog::error::Result;
use dialog::{DialogBuilder, DialogColors, DialogReturnValue};
use tracing::Level;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use tracing_appender::rolling::{RollingFileAppender, Rotation};

fn main() -> Result<()> {

    // region:    -- Tracing Setup

    let filter = EnvFilter::from_default_env();
    // Optionally, you can add a file appender to log to a file.
    let file_appender = RollingFileAppender::new(Rotation::HOURLY, "/home/shane/logs", "dialog.log");
    
    let subscriber = FmtSubscriber::builder()
        .with_writer(file_appender)
        .with_max_level(Level::TRACE)
        .with_env_filter(filter)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;
    
    // endregion: -- Tracing Setup

    // region:    -- Setup Dialog
    
    let borders = Borders {
        top: BorderStyle::Double,
        left: BorderStyle::Single,
        right: BorderStyle::Single,
        bottom: BorderStyle::Double,
        split: BorderStyle::Double
    };

    let builder = DialogBuilder::new(" Add Contact ")
    .set_borders(borders)
    .set_margin((4, 1).into());

    let colors = DialogColors {
        border: Colors::new(Color::DarkGrey, Color::Blue),
        fill: Colors::new(Color::DarkGrey, Color::Blue),
        overlay: Colors::new(Color::White, Color::DarkGrey),
        fields: FieldColors::new(
            Colors::new(Color::DarkGrey, Color::Blue),
            Colors::new(Color::White, Color::Blue),
            Colors::new(Color::White, Color::Blue)
        ),
        buttons: ButtonColors::new(
            Colors::new(Color::White, Color::Blue),
            Colors::new(Color::White, Color::Blue)
        )
    };

    let builder = builder
        .add_control(Control::Button(Button::new("OK", Some(4), dialog::DialogResult::Ok, dialog::ButtonCount::One)))
        .add_control(Control::Button(Button::new("Cancel", Some(5), dialog::DialogResult::Cancel, dialog::ButtonCount::Two)));
    
    let mut dialog = builder
        .add_control(Control::TextField(Field::new("First Name", 15, 15, Some(0), 0)))
        .add_control(Control::TextField(Field::new("Last Name", 15, 15, Some(1), 1)))
        .add_control(Control::TextField(Field::new("Company Name", 15, 40, Some(2), 2)))
        .add_control(Control::TextField(Field::new("Phone Number", 15, 15, Some(3), 3)))
        .set_colors(colors)
        .set_overlay(true)
        .build();   
    dialog.resize()?;

    // endregion: -- Setup Dialog

    // region:    -- Setup RawMode

    stdout()
        .queue(EnterAlternateScreen)?
        .flush()?;
    
    enable_raw_mode()?;

    // endregion: -- Setup RawMode
    
    let mut result = DialogReturnValue::default();

    dialog.show()?;
    
    loop {
        if poll(Duration::from_secs(0))? {
            if let Event::Key(event) = read()? {
                match (event.code, event.modifiers) {
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => break,
                    (code, modifiers) => {
                        result = dialog.handle_input(code, modifiers)?;
                    }
                }
            }
            if result.should_quit { break; }
        }
    }

    let data = dialog.get_data();

    // region:    -- End RawMode
    disable_raw_mode()?;
    stdout().queue(LeaveAlternateScreen)?.flush()?;
    // endregion: -- End RawMode
    
    println!("Exited with: {:?}", data);
    Ok(())
}

