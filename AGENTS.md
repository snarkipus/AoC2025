# AGENTS.md

This file provides guidance to agents when working with code in this repository.

## Build/Lint/Test Commands
- Use `just work day-XX part1` for development workflow (watches files, runs check, test, lint)
- Use `just test day-XX part1` to run specific tests with cargo nextest
- Use `just lint day-XX` for clippy linting
- Use `just run day-XX part1` to execute the production binary in release mode
- Use `just bench day-XX part1` for benchmarking with divan
- `just create day-XX` to generate new day from daily-template and fetch input

## Code Style
- Max line width: 60 characters (rustfmt.toml)
- Function call width: 40 characters
- Comment width: 80 characters
- Imports granularity: Crate level
- Process functions: Use #[tracing::instrument] and return miette::Result<String>

## Project-Specific Patterns
- Each day is a separate Cargo crate in workspace (members = ["day-*"])
- Input files (input1.txt, input2.txt) downloaded via `just get-input day-XX` (requires SESSION env var in .env)
- Benchmarks use divan with harness=false in Cargo.toml [[bench]]
- Template generation: cargo generate from daily-template
- Testing: cargo nextest with rstest
- Error handling: miette with fancy features