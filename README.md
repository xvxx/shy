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


## Screenies

| Nav | Search |
| --- | --- |
| <img src="./img/screen1.jpeg"> |<img src="./img/screen2.jpeg">  |

## TODO

- [ ] correctly parse .ssh/config
- [ ] config parsing
- [ ] show actual hostnames in status bar
- [ ] screencast for README

[cargo]: https://rustup.rs/