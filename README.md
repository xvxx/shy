# shy

`shy` is a lil console ui for quickly connecting to an ssh server. It
parses your `~/.ssh/config` file and displays all your "Host" patterns,
allowing you to quickly select one using a readline-ish prompt.

Why not just setup auto-completion on `ssh` in your shell? Do it! This
project is mostly for fun, and to provide a some funky visuals.

## TODO

### Core

- [ ] correctly parse .ssh/config
- [ ] readline prompt
- [ ] jump to host based on prompt
- [ ] fancy, fish-colored prompt
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
