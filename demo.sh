#!/usr/bin/env bash
alias phone="$(pwd)/target/release/rust-crdt-talk --store-path phone.json"
alias computer="$(pwd)/target/release/rust-crdt-talk --store-path computer.json"

sync_both() {
  phone merge computer.json
  computer merge phone.json
}
