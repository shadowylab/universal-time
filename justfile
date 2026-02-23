#!/usr/bin/env just --justfile

fmt:
    cargo +nightly fmt -- --config format_code_in_doc_comments=true

check:
    cargo check
    cargo check --no-default-features

clippy:
    cargo clippy
    cargo clippy --no-default-features

test:
    cargo test
    cargo test --no-default-features

precommit: fmt check clippy test
