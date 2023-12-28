run component:
  cargo run --features bevy/dynamic_linking -p {{component}}

editor:
  CONFIG_FILE=./editor/settings.ron just run editor

game:
  CONFIG_FILE=./editor/settings.ron just run game

test component: 
  cargo test -- --nocapture

test-editor:
  just test editor

test-game:
  just test game

test-all: test-game test-editor

format-check:
  cargo fmt --all -- --check

clippy-check:
  cargo clippy --all-targets

format:
  cargo fmt --all

clippy:
  cargo clippy --all-targets --fix

