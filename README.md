# shy

`shy` is a lil console ui for quickly connecting to an ssh server. It
parses your `~/.ssh/config` file and displays all your "Host" patterns,
allowing you to quickly select one using a readline-ish prompt.

## Keyboard Shortcuts

| **Shortcut**     | **Nav Mode**        | **Search Mode**                    |
| ---------------- | ------------------- | ---------------------------------- |
| `i`, `s`         | Enter search mode   |                                    |
| `r`              | Refresh             |                                    |
| `ctrl-c`, `ESC`  | Quit                | Clear Input, then Exit Search Mode |
| `up`, `ctrl-p`   | Move selection up   | Jump to previous match             |
| `down`, `ctrl-n` | Move selection down | Jump to next match                 |

## TODO

### Core

- [ ] correctly parse .ssh/config
- [ ] show actual hostnames in status bar

### Fancy

- [ ] man page
- [ ] screenshot for README
- [ ] screencast for README
- [ ] publish as crate
- [ ] usage in README
- [ ] -h
- [ ] -v
- [ ] installation in README
- [ ] tests
