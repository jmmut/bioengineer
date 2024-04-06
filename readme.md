# Bioengineer
[![deployment status badge](https://github.com/jmmut/bioengineer/actions/workflows/release.yml/badge.svg)](https://github.com/jmmut/bioengineer/actions)

## Play the game

https://jmmut.itch.io/bioengineer

At the moment you can play in the browser (playable with mouse and keyboard) or download native versions.

## Compile and run the game

[Install rust](https://www.rust-lang.org/tools/install).

Then, clone the repo:
```
git clone git@github.com:jmmut/bioengineer.git
cd bioengineer
```

and then, simply (`-r` is for compiling in release mode, i.e. with optimizations):
```
cargo run -r
```

That should work in Mac and Linux, and I think Windows too. If this doesn't
work, refer to [the macroquad documentation](https://github.com/not-fl3/macroquad/#linux) or to [the github actions in this repo](.github/workflows/build.yml).
You might need to install some system libraries.
