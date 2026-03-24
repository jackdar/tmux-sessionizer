# tmux-sessionizer

A lightweight tmux session manager for jumping between project directories and existing tmux sessions.

It builds a list of:
- existing tmux sessions
- directories discovered from your config

...then lets you pick one via `fzf`. If you pick a directory, it creates (or switches to) a tmux session for that folder.

## Requirements

- `tmux`
- `fzf` (used for the interactive picker)

## Install

From source (this repo):

```bash
cargo install --path .
```

## Usage

Interactive picker:

```bash
tmux-sessionizer
```

Jump directly to a directory (creates/switches the session for that folder):

```bash
tmux-sessionizer /path/to/project
```

Use a custom config file:

```bash
tmux-sessionizer --config /path/to/config.toml
```

## Config

Default path:

`~/.config/tmux-sessionizer/config.toml`

If the default config file does not exist, the app falls back to an internal default.

Example `config.toml`:

```toml
# List of base directories to scan.
# Each entry is "path" or "path:depth".
directories = [
  "~/code:2",
  "~/work:3",
]

show_hidden = false
follow_symlinks = false

# If false, directories are shown as "~"-contracted paths in the picker.
# If true, full expanded paths are shown.
expand_tilde = false
```

Notes:
- `depth` controls how deep to scan under each base directory.
- Session names are derived from the directory name (final path segment); dots (`.`) are replaced with underscores (`_`).

## Troubleshooting

- If the picker does not open, confirm `fzf` is installed and on your `PATH`.
- If no tmux sessions show up, the tmux server may not be running yet; selecting a directory will start a session.
