# Project Coding Rules (Non-Obvious Only)
- Process functions MUST use #[tracing::instrument] and return miette::Result<String>
- Benchmarks require `harness = false` in Cargo.toml [[bench]]
- Use `just` commands instead of raw `cargo` for workflow consistency