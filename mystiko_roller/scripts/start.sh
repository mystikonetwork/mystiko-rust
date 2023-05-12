#!/bin/bash
rm ../target/debug/mystiko_roller
cargo build

ROLLER_HOME_PATH=/Users/zhangrui/ax-project/rust/mystiko-rust/mystiko_roller/scripts/roller  \
  COIN_MARKET_CAP_API_KEY=773d38a7-7c78-4790-94f5-2e15fb297ea9 \
  X_SCAN_API_KEY=xxx \
  ROLLER_PRIVATE_KEY=0x2afc95b21d944494e8b739ec7f40a953b46cc67c4f720537bbb3361db14bf781 \
  ../target/debug/mystiko_roller
