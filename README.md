# Chronicle

write your chronicle from command line

## Install

```sh
# from cargo
cargo install chronicle
```

## Usage

```sh
chron
chron help [SUBCOMMAND]
```

**Pro tip:** Consider using shell aliases if these feel too wordy.

```sh
# create new chronicle named "code"
# set the file for persisting the chronicle
chron new code /Users/eagr/logs/code.md

# set vim as default editor
chron config --editor "$(which vim)"
# set VS Code as default editor
chron config --editor "$(which code)"

# events willed be inked in reverse chronological order,
# which is the default
chron config --reverse
# in chronological order
chron config --reverse f

# set timestamp formats for date and hms, which adheres to
# https://docs.rs/chrono/0.4/chrono/format/strftime/index.html
chron config --date "%b %e, %Y"
chron config --time "%I:%M %p"

# recommended workflow: draft -> reword -> ink
chron draft code "[Implemented X](...)"
chron reword code
chron ink code

# wipe off draft buffer of "code"
# erased content can be found in $HOME/.chronicle/backup~/
chron erase code
```

## Configuration

You can also configure Chronicle by modifying the config file, which defaults to

* on Unix: $HOME/.chronicle/config.toml
* on Windows: %USERPROFILE%\\.chronicle\config.toml

```toml
# ===== global configuration =====

# editor for rewording
editor = "/usr/bin/vim"

# whether to list events in reverse chronological order,
# which defaults to
reverse = true

# timestamp date format, which defaults to
date = "%Y-%m-%d"

# timestamp hms format, which defaults to
time = "%H:%M"

# ===== per-chronicle configurations =====

# config options specific to chronicle "code",
# which override their global counterpart
[chronicle.code]

# === required fields ===

# path of file for persisting chronicle "code"
store = "/Users/eagr/logs/code.md"

# === optional fields ===

# options below will inherit from their global counterpart
# if you leave them out
reverse = true  # or false
date = "..."
time = "..."
```

## License

Except as otherwise noted, the project is distributed under
[MIT](./LICENSE-MIT) and [Apache License 2.0](./LICENSE-Apache).
