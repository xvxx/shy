# shy

`shy` is a text ui for quickly connecting to an ssh server. It parses
your `~/.ssh/config` file and displays all known hosts, allowing you
to quickly open one using a readline-powered prompt.

Why not just setup auto-completion on `ssh` in your shell? Do it! This
project is mostly for fun, and to provide a visual overview of all
your hosts.

## TODO

### Core

- [ ] parse .ssh/config into an AST
- [ ] display hosts in a TUI
- [ ] up/down between hosts
- [ ] readline prompt
- [ ] jump to host based on prompt
- [ ] ENTER to connect to host

### Fancy

- [ ] man page
