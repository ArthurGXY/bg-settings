# A wallpaper orchestrator for wayland

It is always been a painful process setting up wallpapers on wayland. This program aims to support multiple backends (swaybg, 
hyprpaper, awww, mpvpaper, etc.) and provide a unified interface for setting wallpapers.

## Features Roadmap

- [x] cli interface (mvp)
- [ ] static wallpaper backends support
    - [x] swaybg backend
    - [ ] hyprpaper backend
- [x] multi-monitor modes
- [ ] gif/video backends
    - [ ] mpvpaper
    - [ ] awww
- [ ] graphical interface (tauri)
- [ ] advanced modes
    - [ ] custom commands for other backends
- [ ] effects
  - [ ] Timed auto-swapping

## Usage

This `bg-cli` command will setup all your monitor (outputs)
with `swaybg` (if installed)
and choose a random picture for you from <media-path> as of `v0.1.0`.

Currently, all media in your media-path, if it is a directory,
that are supported by the chosen backend will be scanned since
recursion is not yet effective. This will be in `v0.1.1`.

Choosing different subsets of wallpaper for each output will be 
in `v0.1.2`.

```bash
# setup for all outputs with files in media-path.
bg-cli <media-path> setup

# setup for selected outputs:
bg-cli <media-path> setup [output1, output2]
```

The cli can recursively scan and list the media in given media-path:
```sh
bg-cli <media-path> list media # list all media

bg-cli <media-path> list image[s] # list static images

bg-cli <media-path> list animated # list animated images (gif/apng)

bg-cli <media-path> list video[s] # list videos
```

Besides, it also lists output name or wayland seat info:

```sh
bg-cli list output[s]

bg-cli list seat[s]
```

CLI program source code is at `crate/bg-cli`.

- GUI program is not yet available. It will only be so after
  the CLI program implements all features I planned in README.md.