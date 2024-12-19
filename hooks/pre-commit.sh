#!/bin/bash
npx --no-install -- commitlint --verbose --edit $1 && cargo fmt && cargo clippy && cargo check && cargo test