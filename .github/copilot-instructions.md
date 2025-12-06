# AoC2025 - Advent of Code 2025

This is a Rust workspace for solving Advent of Code 2025 challenges. Each day's puzzle is implemented as a separate workspace member with its own binary, tests, and benchmarks.

## Tech Stack

- **Language**: Rust (edition 2024)
- **Build System**: Cargo (workspace-based)
- **Task Runner**: just (justfile for common tasks)
- **Testing**: cargo-nextest, rstest
- **Benchmarking**: divan
- **Development**: cargo-watch for live reloading
- **Template Generation**: cargo-generate

## Key Dependencies

- `itertools`: Iterator utilities for puzzle solving
- `nom`: Parser combinator library for input parsing
- `tracing` / `tracing-subscriber`: Logging and debugging
- `miette`: Error handling with fancy diagnostics
- `rayon`: Parallel iterators for performance
- `glam`: Vector math utilities
- `nom_locate`: Location tracking for nom parsers

## Project Structure

```
.
├── Cargo.toml              # Workspace manifest
├── daily-template/         # Template for generating new day challenges
│   ├── src/
│   │   ├── lib.rs         # Module exports
│   │   ├── part1.rs       # Part 1 solution
│   │   ├── part2.rs       # Part 2 solution
│   │   └── bin/           # Binary entry points
│   └── benches/           # Benchmark files
├── day-XX/                # Individual day implementations (generated)
├── scripts/               # Utility scripts (e.g., fetch AoC input)
├── justfile               # Task definitions
└── rustfmt.toml           # Code formatting rules
```

## Development Workflow

### Creating a New Day's Puzzle

Use the `just create` command to generate a new day's directory from the template:

```bash
just create day-XX
```

This will:
1. Generate a new workspace member from `daily-template`
2. Fetch the puzzle input from Advent of Code (requires SESSION token in .env)

### Working on a Solution

Use the watch mode for rapid iteration:

```bash
just work day-XX part1
```

This command will:
- Watch for file changes in the day's directory
- Run `cargo check` on changes
- Run tests for the specified part
- Run linting (clippy)

### Running Tests

```bash
# Test a specific part
just test day-XX part1

# Test all parts for a day
cargo nextest run -p day-XX
```

### Linting

```bash
just lint day-XX
```

### Benchmarking

```bash
# Benchmark a specific part
just bench day-XX part1

# Benchmark all puzzles
just bench-all
```

## Coding Guidelines

### Code Style

- Follow the `rustfmt.toml` configuration:
  - Max line width: 60 characters
  - Wrap comments at 50 characters
  - Function call width: 40 characters
  - Crate-level import granularity

### Solution Structure

Each day's solution follows this pattern:

```rust
// part1.rs or part2.rs
#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    // Solution implementation
    todo!("Implement solution")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "test input";
        assert_eq!("expected", process(input)?);
        Ok(())
    }
}
```

### Best Practices

1. **Use `miette::Result<T>` for error handling**: Provides better error messages with context
2. **Add `#[tracing::instrument]` to main functions**: Helps with debugging
3. **Write tests first**: Use the example from the puzzle description
4. **Parse input properly**: Use `nom` for complex parsing, `lines()` + `split()` for simple cases
5. **Consider performance**: Use `rayon` for parallelizable operations, but profile first
6. **Keep part1 and part2 separate**: Don't refactor shared code until both parts are complete
7. **Use itertools**: Common patterns like `.chunks()`, `.combinations()`, `.permutations()` are built-in

### Input Files

- `input1.txt`, `input2.txt`: Puzzle input files (gitignored)
- Place in the day's directory root
- Fetched automatically with `just get-input day-XX` (requires SESSION in .env)

### Environment Variables

Create a `.env` file in the repository root with:

```
SESSION=your_advent_of_code_session_cookie
```

To get your session cookie:
1. Go to https://adventofcode.com/2025/day/1/input
2. Open browser DevTools → Application tab
3. Look under Cookies → https://adventofcode.com
4. Copy the `session` value

## Testing Strategy

1. **Example Tests**: Use the example from the puzzle description
2. **Edge Cases**: Test empty input, single element, maximum values
3. **Integration Tests**: Test the full `process()` function
4. **Unit Tests**: Test helper functions separately

## Build & CI

```bash
# Check compilation without building
cargo check

# Build all workspace members
cargo build --release

# Run all tests
cargo nextest run

# Run all benchmarks
cargo bench
```

## References

- [Advent of Code 2025](https://adventofcode.com/2025)
- [Rust Book](https://doc.rust-lang.org/book/)
- [nom Documentation](https://docs.rs/nom/)
- [itertools Documentation](https://docs.rs/itertools/)
- [miette Documentation](https://docs.rs/miette/)

## Notes for AI Assistants

- When creating solutions, prioritize clarity over cleverness
- Add comments for complex algorithmic steps
- Don't optimize prematurely - make it work, then make it fast
- Use the test framework that's already set up (rstest for parameterized tests)
- Maintain consistency with the existing template structure
- Each day is independent - don't create cross-day dependencies
