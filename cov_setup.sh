#!/bin/sh

export PATH="./scripts:$PATH"

echo "Installing grcov for test coverage"
cargo install grcov
