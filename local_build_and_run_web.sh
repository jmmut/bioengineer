#!/bin/bash

# Before running this script, you'll need to:
# - add the WebAssembly target in the rust compiler:
#   rustup target add wasm32-unknown-unknown
# - install wasm-pack to use wasm-bindgen
#   cargo install wasm-pack
# - install the local web server:
#   cargo install basic-http-server
# see the readme.md for more info

./recompile_web.sh

# you can also do `basic-http-server export_html/` for faster iteration time if you are
# just changing the html/js, but you will need to copy the built_html/pkg to export_html for the
# webassembly artifact to work
basic-http-server bioengineer_html/
