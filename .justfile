update:
    prek update
    cargo upgrade --incompatible
    cargo update

fmt:
    cargo +nightly fmt
    biome format --write .
    taplo fmt *toml
    just --fmt

check:
    prek run --all-files
    biome ci .
    autocorrect --lint
    cargo deny --workspace --all-features check
    cargo clippy --workspace --all-targets --all-features -- --deny warnings

test:
    cargo nextest run --workspace --all-targets --all-features --locked
