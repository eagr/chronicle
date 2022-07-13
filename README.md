# chronicle

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

```sh
# create new chronicle named "code"
# set the file for persisting the chronicle
chron new code /Users/eagr/logs/code.md

# set vim as default editor
chron config --editor "$(which vim)"
# set VS Code as default editor
chron config --editor "$(which code)"

# set timestamp format for dates, which adheres to
# https://docs.rs/chrono/0.4/chrono/format/strftime/index.html
chron config --date "%b %e, %Y"

# `chron` can be configured per chronicle
# set timestamp format for *time* that applies to "code" only
chron config code --time "%I:%M %p"

# standard workflow: draft -> reword -> ink
chron draft code "[Implemented X](...)"
chron reword code
chron ink code

# wipe off draft buffer of "code"
# erased content can be found in $HOME/.chronicle/backup~/
chron erase code
```

## License

Except as otherwise noted, the project is distributed under
[MIT](./LICENSE-MIT) and [Apache License 2.0](./LICENSE-Apache).
