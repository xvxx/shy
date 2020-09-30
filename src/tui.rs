use crate::{
    color,
    ssh_config::{load_ssh_config, HostMap},
};
use flume::{unbounded, Receiver, Selector};
use std::{
    io::{self, Stdout, Write},
    thread,
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
    pub mode: Mode,
    status: SearchStatus,
    input: String,
    selected: usize,
    offset: usize,
    size: (u16, u16),
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

/// Was the input search successful?
#[derive(PartialEq)]
pub enum SearchStatus {
    Blank,
    Found,
    Missed,
}

impl TUI {
    /// Create a new main view and sets up the terminal.
    pub fn new(config_path: &str) -> io::Result<TUI> {
        Ok(TUI {
            mode: Mode::Nav,
            status: SearchStatus::Blank,
            input: String::new(),
            selected: 0,
            offset: 0,
            size: terminal_size()?,
            hosts: load_ssh_config(config_path)?,
            stdout: Self::setup_terminal()?,
        })
    }

    /// Put the terminal into raw mode, hide the cursor, etc.
    fn setup_terminal() -> io::Result<RawTerminal<Stdout>> {
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
    fn cleanup_terminal(&mut self) -> io::Result<()> {
        self.stdout.suspend_raw_mode()?;
        write!(self.stdout, "{}", ShowCursor)?;
        write!(self.stdout, "{}", ToMainScreen)?;
        self.stdout.flush()?;
        Ok(())
    }

    /// Start thread to listen for keyboard events.
    fn event_thread(&self) -> io::Result<Receiver<Key>> {
        let (sender, receiver) = unbounded();
        thread::spawn(move || loop {
            sender
                .send(io::stdin().keys().next().unwrap().unwrap())
                .unwrap()
        });
        Ok(receiver)
    }

    /// Register signal handler. SIGWINCH (resize) only for now.
    fn signal_thread(&self) -> io::Result<Receiver<Key>> {
        let (sender, receiver) = unbounded();
        unsafe {
            signal_hook::register(signal_hook::SIGWINCH, move || {
                sender.send(Key::F(5)).unwrap()
            })
        }?;

        Ok(receiver)
    }

    /// Main loop. Returns the host we want to SSH to, if any.
    pub fn run(&mut self) -> io::Result<Option<String>> {
        let ux_rx = self.event_thread()?;
        let signal_rx = self.signal_thread()?;

        self.update(None)?;
        self.draw()?;

        while let Ok(event) = Selector::new()
            .recv(&ux_rx, |e| e)
            .recv(&signal_rx, |e| e)
            .wait()
        {
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
    pub fn update(&mut self, event: Option<Key>) -> io::Result<()> {
        if event.is_none() {
            return Ok(());
        }

        match event.unwrap() {
            Key::Ctrl('c') | Key::Esc if self.mode == Mode::Nav => self.mode = Mode::Quit,
            Key::Char('r') | Key::F(5) if self.mode == Mode::Nav => {
                self.size = terminal_size()?;
                // reset offset if the screen grew
                if self.offset > 0 && self.hosts.len() <= self.size.1 as usize {
                    self.offset = 0;
                }
            }
            Key::Char(' ') | Key::PageDown => {
                self.selected += 5;
                if self.selected > self.hosts.len() - 1 {
                    self.selected = self.hosts.len() - 1;
                }
                self.select(self.selected);
            }
            Key::Char('-') | Key::PageUp => {
                if self.selected > 5 {
                    self.selected -= 5;
                } else {
                    self.selected = 0;
                }
                self.select(self.selected);
            }
            Key::Up | Key::Ctrl('p') => self.select_prev(),
            Key::Down | Key::Ctrl('n') => self.select_next(),
            Key::Char('\n') => {
                if self.mode == Mode::Search && self.status == SearchStatus::Missed {
                    // do nothing on a search that doesn't match
                } else if let Some(host) = self.hosts.iter().nth(self.selected) {
                    self.mode = Mode::Launch(host.0.clone());
                } else {
                    return Err(io::Error::new(io::ErrorKind::Other, "can't find host"));
                }
            }
            event if self.mode == Mode::Nav => match event {
                Key::Char('q') => self.mode = Mode::Quit,
                Key::Char('i') | Key::Char('s') | Key::Char('/') | Key::Char('f') => {
                    self.status = SearchStatus::Blank;
                    self.mode = Mode::Search
                }
                _ => {}
            },
            event if self.mode == Mode::Search => self.update_input(event),
            _ => {}
        }

        Ok(())
    }

    /// Search mode-specific keybindings.
    fn update_input(&mut self, event: Key) {
        match event {
            Key::Ctrl('c') | Key::Esc => {
                self.status = SearchStatus::Blank;
                if self.input.is_empty() {
                    self.mode = Mode::Nav;
                } else {
                    self.input.clear();
                }
            }
            Key::Backspace => {
                if !self.input.is_empty() {
                    self.input.truncate(self.input.len() - 1);
                    self.select_search_host();
                }
                if self.input.is_empty() {
                    self.status = SearchStatus::Blank;
                }
            }
            Key::Char(c) => {
                self.input.push(c);
                self.select_search_host();
            }
            _ => {}
        }
    }

    /// Select a host by index.
    fn select(&mut self, i: usize) {
        self.selected = i;
        self.status = SearchStatus::Found;
        if !self.is_visible(self.selected) {
            let rows = self.size.1 as usize - 2;
            if self.selected == 0 {
                self.offset = 0;
            } else if self.selected < self.offset {
                self.offset = self.selected;
            } else if self.selected > rows {
                self.offset = self.selected - rows;
            }
        }
    }

    /// Is the host at the given index visible on screen?
    fn is_visible(&self, i: usize) -> bool {
        i >= self.offset && i < self.offset + (self.size.1 as usize - 1)
    }

    /// Select the previous host (up). If we're in search mode, only
    /// selects a matching host.
    fn select_prev(&mut self) {
        if self.mode == Mode::Search && !self.input.is_empty() {
            let mut i = self.selected;
            let hosts = self.hosts.iter().map(|(h, _)| h).collect::<Vec<_>>();
            while i > 0 {
                i -= 1;
                if let Some(host) = hosts.get(i) {
                    if Self::host_matches(host, &self.input) {
                        self.select(i);
                        return;
                    }
                }
            }
        } else {
            if self.selected == 0 {
                self.selected = self.hosts.len() - 1;
            } else {
                self.selected -= 1;
            }
        }
        if !self.is_visible(self.selected) {
            let rows = self.size.1 as usize - 2;
            if self.selected > self.offset && self.selected > rows {
                self.offset = self.selected - rows;
            } else {
                self.offset = self.selected;
            }
        }
    }

    /// Select the previous host (up). If we're in search mode, only
    /// selects a matching host.
    fn select_next(&mut self) {
        if self.mode == Mode::Search && !self.input.is_empty() {
            let mut i = self.selected;
            let hosts = self.hosts.iter().map(|(h, _)| h).collect::<Vec<_>>();
            while i < hosts.len() {
                i += 1;
                if let Some(host) = hosts.get(i) {
                    if Self::host_matches(host, &self.input) {
                        self.select(i);
                        return;
                    }
                }
            }
        } else {
            if self.selected >= self.hosts.len() - 1 {
                self.selected = 0;
            } else {
                self.selected += 1;
            }
        }
        if !self.is_visible(self.selected) {
            let rows = self.size.1 as usize - 2;
            if self.selected == 0 {
                self.offset = 0;
            } else if self.selected > rows {
                self.offset = self.selected - rows;
            }
        }
    }

    /// Checks the current self.input against hostnames to find and
    /// select a match.
    fn select_search_host(&mut self) {
        for (i, (host, _)) in self.hosts.iter().enumerate() {
            if Self::host_matches(host, &self.input) {
                self.select(i);
                return;
            }
        }

        self.status = SearchStatus::Missed;
    }

    /// Does a given host match our search string? Just a prefix
    /// match, for now.
    fn host_matches(host: &str, search: &str) -> bool {
        host.to_lowercase().starts_with(&search.to_lowercase())
    }

    /// The name of the currently selected host pattern.
    fn selected_name(&self) -> &str {
        if let Some((_, (name, _))) = self
            .hosts
            .iter()
            .enumerate()
            .find(|(i, _)| *i == self.selected)
        {
            name
        } else {
            "shy"
        }
    }

    /// The hostname of the currently selected host pattern. The two
    /// might be different.
    fn selected_hostname(&self) -> &str {
        if let Some((_, (_, hostname))) = self
            .hosts
            .iter()
            .enumerate()
            .find(|(i, _)| *i == self.selected)
        {
            hostname
        } else {
            "shy"
        }
    }

    /// (bg, fg) colors for the prompt
    fn prompt_colors(&self) -> (&str, &str) {
        match self.status {
            SearchStatus::Blank => (color::WhiteBG.as_ref(), color::Black.as_ref()),
            SearchStatus::Found => (color::GreenBG.as_ref(), color::Black.as_ref()),
            SearchStatus::Missed => (color::RedBG.as_ref(), color::White.as_ref()),
        }
    }

    /// Draw the ui
    pub fn draw(&self) -> io::Result<()> {
        let (_cols, rows) = self.size;
        let mut stdout = io::stdout();

        if self.mode == Mode::Search {
            let (bg, fg) = self.prompt_colors();
            let hint = if self.status == SearchStatus::Found {
                &self.selected_name()[self.input.len()..]
            } else {
                ""
            };

            write!(
                stdout,
                "{}{}{}{}{}{}{}{}{}{}{}{}{}",
                ClearAll,
                Goto(1, rows),
                bg,
                fg,
                ClearLine,
                color!(Bold),
                ">> ",
                self.input,
                color!(Reset),
                bg,
                fg,
                hint,
                color!(Reset),
            )?;
        } else {
            write!(
                stdout,
                "{}{}{}{}{}{}",
                ClearAll,
                Goto(1, rows),
                color!(MagentaBG),
                color!(Yellow),
                ClearLine,
                color_string!(self.selected_hostname(), MagentaBG, Yellow, Bold)
            )?;
        }

        let mut row = 1;
        for (i, (host, _config)) in self.hosts.iter().enumerate().skip(self.offset) {
            if i >= self.offset + (rows as usize - 1) {
                break;
            }

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
