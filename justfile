run component:
  cargo run --features bevy/dynamic_linking -p {{component}}

editor:
  CONFIG_FILE=./editor/settings.ron just run editor

game:
  CONFIG_FILE=./editor/settings.ron just run game

test component test="": 
  cargo test {{test}} -p {{component}} -- --nocapture

test-editor test="":
  just test editor {{test}}

test-game test="":
  just test game {{test}}

test-shared test="":
  just test shared {{test}}

test-all: test-game test-editor test-shared

format-check:
  cargo fmt --all -- --check

clippy-check:
  cargo clippy --all-targets

format:
  cargo fmt --all

clippy:
  cargo clippy --all-targets --fix

