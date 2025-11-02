# https://just.systems
set shell := ["pwsh.exe",  "-NoProfile", "-C"]
default:
  cargo run --features bevy/dynamic_linking

build: 
  cargo build
  
