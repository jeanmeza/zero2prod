# Zero to Production in Rust

## Buld Docker Image

```bash
cargo sqlx prepare --workspace
docker build --tag zero2prod --file Dockerfile .
```

## Packages to install

- cargo install cargo-watch
- cargo install cargo-tarpaulin
- rustup component add clippy
- rustup component add rustfmt
- cargo install cargo-audit
- cargo install --version="~0.7" sqlx-cli --no-default-features --features rustls,postgres
- sudo apt-get install postgresql-client
- config = "0.13"
- cargo add tracing --features log
  `tracing expands upon logging-style diagnostics by allowing libraries and applications to record structured events with additional information about temporary and causality - unlike a log message, a span in tracing has a beginning and end time, may be entered and exited by the flow of execution, and may exist within a nested tree of similar spans.`
- cargo install cargo-udeps (to remove unused dependencies run with: cargo +nightly udeps)
- cargo install bunyan : to prettify the outputted logs 'cargo test test_name | bunyan'
