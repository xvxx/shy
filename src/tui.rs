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
    Nav,
    Quit,
    Launch(String),
}

impl TUI {
    /// Create a new main view and sets up the terminal.
    pub fn new() -> Result<TUI, io::Error> {
        Ok(TUI {
            mode: Mode::Nav,
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

    /// Main loop. Returns the host we want to SSH to, if any.
    pub fn run(&mut self) -> Result<Option<String>, io::Error> {
        self.hosts = load_ssh_config()?;
        self.update(None)?;
        self.draw()?;

        while let Some(Ok(event)) = io::stdin().keys().next() {
            self.update(Some(event))?;
            match self.mode {
                Mode::Quit => break,
                Mode::Launch(ref host) => return Ok(Some(host.clone())),
                _ => self.draw()?,
            }
        }

        Ok(None)
    }

    /// Update our state in response to key presses.
    pub fn update(&mut self, event: Option<Key>) -> Result<(), io::Error> {
        if event.is_none() {
            return Ok(());
        }

        match event.unwrap() {
            Key::Char('q') => self.mode = Mode::Quit,
            Key::Ctrl('c') if self.mode == Mode::Nav => self.mode = Mode::Quit,
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
            event if self.mode == Mode::Search => self.update_input(event),
            _ => {}
        }

        Ok(())
    }

    /// Search mode-specific keybindings.
    fn update_input(&mut self, event: Key) {
        match event {
            Key::Ctrl('c') | Key::Esc => {
                self.input.clear();
                self.mode = Mode::Nav;
            }
            Key::Backspace => {
                if !self.input.is_empty() {
                    self.input.truncate(self.input.len() - 1);
                }
            }
            Key::Char(c) => {
                self.input.push(c);
                self.search_for_host();
            }
            _ => {}
        }
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

    /// Checks the current self.input against hostnames to find a match.
    fn search_for_host(&mut self) {
        for (i, (host, _)) in self.hosts.iter().enumerate() {
            if host.to_lowercase().starts_with(&self.input.to_lowercase()) {
                self.selected = i;
            }
        }
    }
}

/// Try to always clean up the terminal.
impl Drop for TUI {
    fn drop(&mut self) {
        let _ = self.cleanup_terminal();
    }
}
