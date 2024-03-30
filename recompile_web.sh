#!/bin/bash

set -euo pipefail

# Don't run this manually, run local_build_and_run.sh instead to run the project locally.
# This script exists just to make sure the CI and the local building does the same.

# for this script to work, you need to do `rustup target add wasm32-unknown-unknown`

cargo build --release --target wasm32-unknown-unknown --bin bioengineer
mkdir -p bioengineer_html
# the folder export_html contains the html wrapper so that the wasm can be used
cp -r export_html/* bioengineer_html/
cp -r target/wasm32-unknown-unknown/release/*.wasm bioengineer_html/
cp -r assets/ bioengineer_html/

