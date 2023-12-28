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
