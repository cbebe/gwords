#!/bin/sh

# https://github.com/mozilla/grcov

getProfRaw() {
	export RUSTFLAGS="-Z instrument-coverage"
	cargo build
	export LLV_PROFILE_FILE="gwords-%p-%m.profraw"
	cargo test
}

export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
export RUSTDOCFLAGS="-Cpanic=abort"

cargo build
cargo test
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
