# Changelog

All notable changes to this project will be documented in this file.

##  [0.1.0] - 2026-01-01

Happy New Year! And this is the very first release.

The binary is in the release files, now I only have x86_64 support.

### New

This cli command will setup all your monitor (outputs) 
with `swaybg` (if installed) 
and choose a random picture for you from <media-path>.

Currently, all media in your media-path, if it is a directory,
that are supported by the chosen backend will be scanned since
recursion is not yet effective. This will be in `v0.1.1`.

Choosing different subsets of wallpaper will be in `v0.1.2`.

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

## [0.1.1] - 2026-01-06

### New
The recursion control parameters are effective now.
```
-r, --recursive
          
-m, --max-recurse-depth <MAX_RECURSE_DEPTH>
    max recursion depth, where -1 means no limit (default=-1) [default: -1]
```
