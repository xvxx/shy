#[macro_use]
mod color;

use std::{
    io::{self, Stdout, Write},
    panic,
};
use termion::{
    clear::{All as ClearAll, CurrentLine as ClearLine},
    cursor::{Goto, Hide as HideCursor, Show as ShowCursor},
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
    screen::{ToAlternateScreen, ToMainScreen},
    terminal_size,
};

fn main() -> Result<(), io::Error> {
    better_panic::install();
    setup_panic_hook();

    let mut stdout = setup_terminal()?;

    update()?;
    draw()?;

    while let Some(Ok(event)) = io::stdin().keys().next() {
        write!(stdout, "{}{}event: {:?}", Goto(1, 7), ClearLine, event)?;
        stdout.flush()?;

        if event == Key::Char('q') || event == Key::Ctrl('c') {
            break;
        }
    }

    shutdown_terminal()?;
    Ok(())
}

/// Switch to alternate mode, set colors, hide cursor.
fn setup_terminal() -> Result<RawTerminal<Stdout>, io::Error> {
    let mut stdout = io::stdout().into_raw_mode()?;
    write!(stdout, "{}", ToAlternateScreen)?;
    write!(stdout, "{}", HideCursor)?;
    write!(stdout, "{}", ClearAll)?;
    write!(stdout, "{}", Goto(1, 1))?;
    stdout.flush()?;
    Ok(stdout)
}

/// Restore terminal state to pre-launch.
fn shutdown_terminal() -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    write!(stdout, "{}", ShowCursor)?;
    write!(stdout, "{}", ToMainScreen)?;
    stdout.flush()?;
    Ok(())
}

/// We need to cleanup the terminal before exiting, even on panic!
fn setup_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        let _ = shutdown_terminal();
        better_panic::Settings::auto().create_panic_handler()(panic_info);
    }));
}

/// Update our state in response to key presses.
fn update() -> Result<(), io::Error> {
    Ok(())
}

/// Draw the app.
fn draw() -> Result<(), io::Error> {
    let (cols, rows) = terminal_size()?;
    let mut stdout = io::stdout();
    write!(stdout, "hi mom")?;
    write!(stdout, "{}", Goto(1, 3))?;
    write!(stdout, "term is {}x{}", cols, rows)?;
    stdout.flush()?;
    Ok(())
}
