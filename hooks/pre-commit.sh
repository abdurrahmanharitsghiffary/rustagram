#!/bin/bash
npx commitlint --edit $1 && cargo fmt && cargo clippy && cargo check && cargo test
