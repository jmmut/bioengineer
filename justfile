
alias x := experiments
experiments:
    cargo run --bin experiments

xe:
    cargo run --bin experiments -- egui
xm:
    cargo run --bin experiments -- mq

alias t := update-tileset
update-tileset:
    cp assets/image/tileset.png.ln assets/image/tileset.png
