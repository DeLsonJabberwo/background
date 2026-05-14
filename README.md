# Background

A collection of desktop background/wallpaper setter CLI tools implemented in multiple languages.

## Implementations

| Repository | Language | Target | Description |
|------------|----------|--------|-------------|
| [background-feh](https://github.com/DeLsonJabberwo/background-feh) | Rust | X11 | Original implementation using `feh` |
| [background-way-go](https://github.com/DeLsonJabberwo/background-way-go) | Go | Wayland | Wayland port written in Go |
| [background-way-zig](https://github.com/DeLsonJabberwo/background-way-zig) | Zig | Wayland | Lightweight Wayland implementation using `swaybg` |

## About

All implementations provide a simple CLI for changing your desktop background from any location using images from a user-defined directory.

Common features across implementations:
- Set a specific wallpaper by name
- Restore the last used wallpaper
- List available wallpapers
- Simple JSON configuration
