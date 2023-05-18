fmt:
    cargo fmt
    prettier --write .
    taplo fmt *toml
    just --fmt --unstable

update:
    cargo upgrade
    cargo update

check:
    pre-commit run --all-files
    cargo outdated --workspace --exit-code 1
    cargo clippy --workspace --all-targets -- --deny clippy::all

build:
    cargo build --workspace --all-targets
