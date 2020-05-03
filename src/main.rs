use flume::{unbounded, Receiver};
use std::{
    io::{self, Write},
    thread,
};
use termion::{
    cursor::{Goto, Hide as HideCursor, Show as ShowCursor},
    event::Key,
    input::TermRead,
    raw::IntoRawMode,
    screen::{ToAlternateScreen, ToMainScreen},
    terminal_size,
};

fn main() -> Result<(), io::Error> {
    setup_terminal()?;
    let (cols, rows) = terminal_size()?;

    println!("hi mom");
    write!(stdout, "{}", Goto(1, 3))?;
    println!("term is {}x{}", cols, rows);
    std::thread::sleep(std::time::Duration::from_secs(4));

    write!(stdout, "{}", ShowCursor)?;
    write!(stdout, "{}", ToMainScreen)?;
    Ok(())
}

fn setup_terminal() -> Result<(), io::Error> {
    let mut stdout = io::stdout().into_raw_mode()?;
    write!(stdout, "{}", ToAlternateScreen)?;
    write!(stdout, "{}", HideCursor)?;
    write!(stdout, "{}", Goto(1, 1))?;
    Ok(())
}

fn shutdown_terminal() -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    write!(stdout, "{}", ShowCursor)?;
    write!(stdout, "{}", ToMainScreen)?;
    Ok(())
}

fn setup_ui_events() -> Receiver<Key> {
    let (sender, receiver) = unbounded();
    thread::spawn(move || loop {
        sender
            .send(io::stdin().keys().next().unwrap().unwrap())
            .unwrap();
    });

    receiver
}

fn setup_ctrl_c() -> Receiver<()> {
    let (sender, receiver) = unbounded();
    ctrlc::set_handler(move || {
        sender.send(()).unwrap();
    })
    .unwrap();

    receiver
}
