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
    let mut stdout = setup_terminal()?;
    setup_panic_hook();
    let mut selected = 0;
    let hosts = vec!["testing.io", "crates.io", "arch-dev"];

    update()?;
    draw(&hosts, selected)?;

    while let Some(Ok(event)) = io::stdin().keys().next() {
        write!(stdout, "{}{}event: {:?}", Goto(1, 7), ClearLine, event)?;
        stdout.flush()?;

        match event {
            Key::Char('q') | Key::Ctrl('c') => break,
            Key::Up | Key::Ctrl('p') => {
                if selected == 0 {
                    selected = hosts.len() - 1;
                } else {
                    selected -= 1;
                }
            }
            Key::Down | Key::Ctrl('n') => {
                if selected >= hosts.len() - 1 {
                    selected = 0;
                } else {
                    selected += 1;
                }
            }
            Key::Char('\n') => {
                shutdown_terminal()?;
                println!("Connecting to {}", hosts[selected]);
                return Ok(());
            }
            _ => {}
        }

        draw(&hosts, selected)?;
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
    let stdout = io::stdout();
    stdout.into_raw_mode()?.suspend_raw_mode()?;
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
        println!("{}", panic_info);
    }));
}

/// Update our state in response to key presses.
fn update() -> Result<(), io::Error> {
    Ok(())
}

/// Draw the app.
fn draw(hosts: &[&str], selected: usize) -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    write!(
        stdout,
        "{}{}{}",
        ClearAll,
        Goto(1, 1),
        color_string!("shy", Magenta, Bold)
    )?;

    let mut row = 3;
    for (i, host) in hosts.iter().enumerate() {
        write!(
            stdout,
            "{}{}",
            Goto(1, row),
            if i == selected {
                format!("> {}", color_string!(host, Yellow, Bold))
            } else {
                format!("  {}", color_string!(host, White))
            }
        )?;
        row += 1;
    }

    stdout.flush()?;
    Ok(())
}
