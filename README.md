# shy

`shy` is a lil console ui for quickly connecting to an ssh server. It
parses your `~/.ssh/config` file and displays all your "Host" patterns,
allowing you to quickly select one using a readline-ish prompt.

## Install

If you have [cargo] installed, installation is a breeze:

    cargo install shy

Assuming you have `~/.cargo/bin` in your `$PATH` and a `~/.ssh/config`
file, you can now run:

    shy

## Keyboard Shortcuts

| **Shortcut**       | **Nav Mode**        | **Search Mode**                    |
| ------------------ | ------------------- | ---------------------------------- |
| `i`, `s`, `f`, `/` | Enter search mode   |                                    |
| `r`                | Refresh             |                                    |
| `ctrl-c`, `ESC`    | Quit                | Clear Input, then Exit Search Mode |
| `up`, `ctrl-p`     | Move selection up   | Jump to previous match             |
| `down`, `ctrl-n`   | Move selection down | Jump to next match                 |

## TODO

### Core

- [ ] correctly parse .ssh/config
- [ ] show actual hostnames in status bar

### Fancy

- [ ] screenshot for README
- [ ] screencast for README
- [ ] publish as crate
- [ ] usage in README
- [ ] installation in README
- [ ] tests

[cargo]: https://rustup.rs/