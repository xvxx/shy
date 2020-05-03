use crate::ssh_config::{load_ssh_config, HostMap};
use std::{
    io::{self, Stdout, Write},
    os::unix::process::CommandExt,
    panic,
    process::Command,
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

/// App state.
pub struct TUI {
    mode: Mode,
    input: String,
    selected: usize,
    hosts: HostMap,
    stdout: RawTerminal<Stdout>,
}

/// UI mode
#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Search,
    Navigate,
    Quit,
    Launch(String),
}

impl TUI {
    /// Create a new main view and sets up the terminal.
    pub fn new() -> Result<TUI, io::Error> {
        Ok(TUI {
            mode: Mode::Navigate,
            input: String::new(),
            selected: 0,
            hosts: HostMap::new(),
            stdout: Self::setup_terminal()?,
        })
    }

    /// Put the terminal into raw mode, hide the cursor, etc.
    fn setup_terminal() -> Result<RawTerminal<Stdout>, io::Error> {
        let mut stdout = io::stdout().into_raw_mode()?;
        write!(stdout, "{}", ToAlternateScreen)?;
        write!(stdout, "{}", HideCursor)?;
        write!(stdout, "{}", ClearAll)?;
        write!(stdout, "{}", Goto(1, 1))?;
        stdout.flush()?;
        Ok(stdout)
    }

    /// Restore the terminal to its prior state.
    /// We run this on drop().
    fn cleanup_terminal(&mut self) -> Result<(), io::Error> {
        self.stdout.suspend_raw_mode()?;
        write!(self.stdout, "{}", ShowCursor)?;
        write!(self.stdout, "{}", ToMainScreen)?;
        self.stdout.flush()?;
        Ok(())
    }

    /// Main loop.
    pub fn run(&mut self) -> Result<(), io::Error> {
        self.hosts = load_ssh_config()?;
        self.update(None)?;
        self.draw()?;

        while let Some(Ok(event)) = io::stdin().keys().next() {
            self.update(Some(event))?;
            if self.mode == Mode::Quit {
                break;
            }
            self.draw()?;
        }

        Ok(())
    }

    /// Update our state in response to key presses.
    pub fn update(&mut self, event: Option<Key>) -> Result<(), io::Error> {
        if event.is_none() {
            return Ok(());
        }
        let event = event.unwrap();

        match self.mode {
            Mode::Navigate => match event {
                Key::Char('q') | Key::Ctrl('c') => self.mode = Mode::Quit,
                Key::Char('i') | Key::Char('s') => self.mode = Mode::Search,
                Key::Up | Key::Ctrl('p') => {
                    if self.selected == 0 {
                        self.selected = self.hosts.len() - 1;
                    } else {
                        self.selected -= 1;
                    }
                }
                Key::Down | Key::Ctrl('n') => {
                    if self.selected >= self.hosts.len() - 1 {
                        self.selected = 0;
                    } else {
                        self.selected += 1;
                    }
                }
                Key::Char('\n') => {
                    if let Some(host) = self.hosts.iter().nth(self.selected) {
                        self.mode = Mode::Launch(host.0.clone());
                    } else {
                        return Err(io::Error::new(io::ErrorKind::Other, "can't find host"));
                    }
                }
                _ => {}
            },
            Mode::Search => match event {
                Key::Ctrl('c') | Key::Esc => {
                    self.input.clear();
                    self.mode = Mode::Navigate;
                }
                Key::Backspace => {
                    if !self.input.is_empty() {
                        self.input.truncate(self.input.len() - 1);
                    }
                }
                Key::Char('\n') => {
                    if let Some(host) = self.hosts.iter().nth(self.selected) {
                        self.mode = Mode::Launch(host.0.clone());
                    } else {
                        return Err(io::Error::new(io::ErrorKind::Other, "can't find host"));
                    }
                }
                Key::Char(c) => {
                    self.input.push(c);
                }
                _ => {}
            },
            _ => {}
        }

        Ok(())
    }

    /// Draw the ui
    pub fn draw(&self) -> Result<(), io::Error> {
        let (_cols, rows) = terminal_size()?;
        let mut stdout = io::stdout();

        let prompt = if self.mode == Mode::Search {
            format!(
                "{}{}{}{}",
                Goto(1, rows - 2),
                ClearLine,
                color_string!(">> ", Bold, White),
                self.input
            )
        } else {
            "".to_string()
        };

        write!(
            stdout,
            "{}{}{}{}{}{}{}",
            ClearAll,
            prompt,
            Goto(1, rows - 1),
            color!(MagentaBG),
            color!(Yellow),
            ClearLine,
            color_string!("shy", MagentaBG, Yellow, Bold)
        )?;

        let mut row = 3;
        for (i, (host, _config)) in self.hosts.iter().enumerate() {
            write!(
                stdout,
                "{}{}",
                Goto(1, row),
                if i == self.selected {
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
}

/// Try to always clean up the terminal.
impl Drop for TUI {
    fn drop(&mut self) {
        let _ = self.cleanup_terminal();
    }
}
