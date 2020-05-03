SHY(1)

# NAME

shy - barebones ssh launcher

# SYNOPSIS

_shy_ [_OPTIONS_]

# DESCRIPTION

_shy_ is a lil console ui for quickly connecting to an ssh server. It
parses your `~/.ssh/config` file and displays all your "Host"
patterns, allowing you to quickly select one using a readline-ish
prompt.

Usually _shy_ is started with no options:

	shy

If you want to use a config file other than `~/.ssh/config`, however,
you can pass a path using the `-c` or `--config` options.

# OPTIONS

_-c_, _--config_ _FILE_
	Use _FILE_ instead of _~/.ssh/config_

_-h_, _--help_
	Print a help summary and exit.

_-v_, _--version_
	Print version information and exit.

# NOTES

If no config file is found, _shy_ will fail to start.

# NAVIGATION

_shy_ has two modes: Navigation mode and Search mode. By default, the
program is in Navigation mode - a simple list view that lets you move
your selected host up or down.

By pressing `i` or `s` the program enters Search mode, allowing you to
quickly jump to a host by typing the beginning of its name.

## NAV MODE KEYBOARD SHORTCUTS

_q_, _Esc_, _Ctrl-c_
	Quit _shy_.

_up arrow_, _Ctrl-p_, _k_
	Select previous host in list.
_down arrow_, _Ctrl-n_, _j_
	Select next host in list.

_Enter_
	Connect to selected host.

_i_, _s_, _/_, _f_
	Enter search mode.

## SEARCH MODE KEYBOARD SHORTCUTS

_Esc_, _Ctrl-c_
	Clear the input, and then exit Search mode.

_up arrow_, _Ctrl-p_, _k_
	Select previous matching host.
_down arrow_, _Ctrl-n_, _j_
	Select next matching host.

_Enter_
	Connect to selected host.

# ABOUT

_shy_ is maintained by chris west, and released under the MIT license.

shy's Gopher hole:
	_gopher://phkt.io/1/shy_
shy's webpage:
	_https://github.com/xvxx/shy_
