# shy

`shy` is a text ui for quickly connecting to an ssh server. It parses
your `~/.ssh/config` file and displays all known hosts, allowing you
to quickly open one using a readline-ish prompt.

Why not just setup auto-completion on `ssh` in your shell? Do it! This
project is mostly for fun, and to provide a visual overview of all
your hosts.

## TODO

### Core

- [x] parse .ssh/config into an AST
- [x] display hosts in a TUI
- [x] up/down between hosts
- [ ] readline prompt
- [ ] just start typing
- [ ] jump to host based on prompt
- [ ] fancy, fish-colored prompt
- [x] ENTER to connect to host
- [ ] show actual hostnames in 2nd column

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
