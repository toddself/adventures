run component:
  cargo run --features bevy/dynamic_linking -p {{component}}

editor:
  CONFIG_FILE=./editor/settings.ron just run editor

game:
  CONFIG_FILE=./editor/settings.ron just run game

test:
  cargo test
