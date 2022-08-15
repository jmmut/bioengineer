# Bioengineer

## Run the game

[Install rust](https://www.rust-lang.org/tools/install).

Then, clone the repo:
```
git clone git@github.com:jmmut/bioengineer.git
cd bioengineer
```

and then, simply:
```
cargo run
```

That should work in Mac and Linux, and I think Windows too. If this doesn't
work, refer to [the macroquad documentation](https://github.com/not-fl3/macroquad/#linux). You might need to
install some system libraries.

## Export the game

To export the game to HTML, do:

```
rustup target add wasm32-unknown-unknown
cargo build -r --target wasm32-unknown-unknown

# the folder export_html contains the html wrapper so that the wasm can be used
cp -r target/wasm32-unknown-unknown/release/*.wasm export_html/
cp -r assets/ export_html/

# you can zip the folder and upload it to itch.io with butler (you'll have to install butler and log in!)
zip -r wasm.zip export_html/*
butler push wasm.zip jmmut/Bioengineer:html5

# or you can run locally with a local http server
cargo install basic-http-server
basic-http-server export_html/
```
