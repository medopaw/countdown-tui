# Countdown TUI

A terminal countdown timer with big digital display, written in Rust.

This is a 100% feature-compatible Rust implementation of [antonmedv/countdown](https://github.com/antonmedv/countdown).

## Install

```sh
cargo install countdown-tui
```

## Usage

Specify duration in format `1h2m3s` or a target time: `02:15pm`, `14:15`.

```sh
countdown-tui 25s
countdown-tui 11:32
```

Add a command with `&&` to run after the countdown.

```sh
countdown-tui 1m30s && echo "Time's up!"
```

Count up from zero.

```sh
countdown-tui --up 30s
```

Announce (via macOS `say` command) last 10 seconds.

```sh
countdown-tui --say 10s
```

Display a title below the countdown timer.

```sh
countdown-tui --title "Coffee Break" 30s
```

## Key bindings

- `Space`: Pause/Resume the countdown
- `Esc` or `Ctrl+C`: Stop the countdown

## Options

```
A terminal countdown timer with big digital display

Usage: countdown-tui [OPTIONS] <DURATION>

Arguments:
  <DURATION>  Duration or target time (e.g., 25s, 1m30s, 14:15, 02:30PM)

Options:
  -u, --up            Count up from zero
  -s, --say           Announce the time left (macOS only)  
  -t, --title <TEXT>  Display title below the countdown
  -h, --help          Print help

Examples:
  countdown-tui 25s
  countdown-tui -title "Coffee Break" 14:15
  countdown-tui 02:15PM
  countdown-tui -up 30s
  countdown-tui -say 10s
```

## Features

- ⏱️  Large ASCII art digital display
- 🕐 Duration format: `25s`, `1m30s`, `1h2m3s`
- 🕒 Time format: `14:15`, `02:30PM`, `10:00AM`
- ⏯️  Pause/resume with spacebar
- 🔄 Count up or count down modes
- 🔊 Voice announcements (macOS)
- 📺 Responsive terminal display
- 🎨 Beautiful Unicode box drawing characters

## Requirements

- Terminal with Unicode support
- macOS (for `--say` voice announcements)

## License

[MIT](LICENSE)

## Related

- [fx](https://github.com/antonmedv/fx) – terminal JSON viewer
- [walk](https://github.com/antonmedv/walk) – terminal file manager  
- [howto](https://github.com/antonmedv/howto) – terminal command LLM helper
- [countdown](https://github.com/antonmedv/countdown) – original Go implementation