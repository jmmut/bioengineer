# Bioengineer
[![build status badge](https://github.com/jmmut/bioengineer/actions/workflows/build.yml/badge.svg)](https://github.com/jmmut/bioengineer/actions)

## Play the game

https://jmmut.itch.io/bioengineer

At the moment you can play in the browser (playable with mouse and keyboard), and the plan is to add downloadable native versions.

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

## Export the game

To export the game to HTML, do:
```
rustup target add wasm32-unknown-unknown
cargo build -r --target wasm32-unknown-unknown

# the folder export_html contains the html wrapper so that the wasm can be used
mkdir -p bioengineer_html
cp -r export_html/* bioengineer_html/
cp -r target/wasm32-unknown-unknown/release/*.wasm bioengineer_html/
cp -r assets/ bioengineer_html/

# you can zip the folder and upload it to itch.io with butler (you'll have to install butler and log in!)
zip -r wasm.zip bioengineer_html/*
butler push wasm.zip jmmut/Bioengineer:html5

# or you can run locally with a local http server
cargo install basic-http-server
basic-http-server bioengineer_html/
```

To cross-compile from Linux to Windows, do:
```
rustup target add x86_64-pc-windows-gnu
cargo build -r --target x86_64-pc-windows-gnu

mkdir -p bioengineer_windows
cp -r assets/ bioengineer_windows/
cp target/x86_64-pc-windows-gnu/release/bioengineer.exe bioengineer_windows/
zip -FS -r bioengineer_windows.zip bioengineer_windows/*

butler push bioengineer_windows.zip jmmut/Bioengineer:windows
```

To export from Linux to Linux, do:
```
cargo build -r

mkdir -p bioengineer_linux
cp -r assets/ bioengineer_linux/
cp target/release/bioengineer bioengineer_linux/
zip -FS -r bioengineer_linux.zip bioengineer_linux/*

butler push bioengineer_linux.zip jmmut/Bioengineer:linux
```
