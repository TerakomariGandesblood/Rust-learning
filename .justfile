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
    cargo clippy --workspace --all-targets -- --deny clippy::all
    autocorrect --lint

build:
    cargo build --workspace --all-targets

test:
    cargo nextest run --workspace --all-targets
