# Advent of Code 2025

My solutions for [Advent of Code 2025](https://adventofcode.com/2025) written in Rust 🦀

## About This Project

This repository contains my personal solutions to the Advent of Code 2025 puzzles, implemented as a Rust workspace with modern tooling for an efficient development experience.

## Acknowledgements

This setup was heavily inspired by [Chris Biscardi](https://github.com/ChristopherBiscardi)'s excellent Advent of Code repositories. If you're interested in Rust, game development with Bevy, or creative coding, definitely check out:

- [Chris's YouTube Channel](https://www.youtube.com/@chrisbiscardi)
- [Chris's Advent of Code Repos](https://github.com/ChristopherBiscardi/advent-of-code)

## Tech Stack

- **Language**: Rust (edition 2024)
- **Build System**: Cargo (workspace-based with multiple crates)
- **Task Runner**: [just](https://github.com/casey/just) (command runner, like make but better)
- **Testing**: [cargo-nextest](https://nexte.st/) (next-generation test runner)
- **Test Framework**: [rstest](https://docs.rs/rstest/) (fixtures and parameterized tests)
- **Benchmarking**: [divan](https://docs.rs/divan/) (statistical benchmarking)
- **Development**: [cargo-watch](https://github.com/watchexec/cargo-watch) (auto-rebuild on file changes)
- **Template Generation**: [cargo-generate](https://github.com/cargo-generate/cargo-generate) (project scaffolding)

### Key Dependencies

- **[itertools](https://docs.rs/itertools/)**: Extended iterator functionality (`.chunks()`, `.combinations()`, `.permutations()`, etc.)
- **[nom](https://docs.rs/nom/)**: Parser combinator library for efficient input parsing
- **[miette](https://docs.rs/miette/)**: Beautiful error handling with fancy diagnostics
- **[tracing](https://docs.rs/tracing/)** / **[tracing-subscriber](https://docs.rs/tracing-subscriber/)**: Structured logging and debugging
- **[rayon](https://docs.rs/rayon/)**: Data parallelism library (currently optional)
- **[glam](https://docs.rs/glam/)**: Mathematics library for vectors/matrices (currently optional)

## Project Structure

```
.
├── Cargo.toml              # Workspace manifest
├── justfile                # Task runner commands
├── rustfmt.toml            # Code formatting configuration
├── scripts/
│   └── get-aoc-input.rs    # Fetch puzzle inputs from AoC
├── daily-template/         # Template for new day solutions
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs         # Module exports
│   │   ├── part1.rs       # Part 1 solution
│   │   ├── part2.rs       # Part 2 solution
│   │   └── bin/           # Binary entry points
│   └── benches/           # Benchmark definitions
└── day-XX/                # Individual day implementations
    ├── Cargo.toml
    ├── input1.txt         # Puzzle input (gitignored)
    ├── input2.txt         # Puzzle input (gitignored)
    ├── src/
    └── benches/
```

Each day is a separate Cargo crate in the workspace, allowing for:
- Independent compilation
- Per-day dependencies
- Isolated benchmarking
- Parallel development

## Prerequisites

Before you start, ensure you have the following installed:

1. **Rust** (latest stable): Install via [rustup](https://rustup.rs/)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **just**: Task runner
   ```bash
   cargo install just
   ```

3. **cargo-nextest**: Fast test runner
   ```bash
   cargo install cargo-nextest
   ```

4. **cargo-watch**: Auto-rebuild on changes (optional, for development workflow)
   ```bash
   cargo install cargo-watch
   ```

5. **cargo-generate**: Project scaffolding (optional, for creating new days)
   ```bash
   cargo install cargo-generate
   ```

## Setup

### 1. Clone the Repository

```bash
git clone https://github.com/snarkipus/AoC2025.git
cd AoC2025
```

### 2. Set Up Session Cookie (for automatic input fetching)

Create a `.env` file in the repository root:

```env
SESSION=your_advent_of_code_session_cookie_here
```

To get your session cookie:
1. Go to [https://adventofcode.com/2025/day/1/input](https://adventofcode.com/2025/day/1/input)
2. Open browser DevTools (F12)
3. Go to the **Application** tab (Chrome) or **Storage** tab (Firefox)
4. Under **Cookies** → `https://adventofcode.com`
5. Copy the value of the `session` cookie
6. Paste it into your `.env` file

**Note**: Keep your session cookie private! The `.env` file is gitignored.

### 3. Build the Project

```bash
cargo build
```

## Development Workflow

### Creating a New Day's Solution

Generate a new day from the template and fetch its input:

```bash
just create day-XX
```

For example, `just create day-03` will:
1. Generate a new `day-03/` directory from `daily-template`
2. Automatically fetch the puzzle input from Advent of Code
3. Create placeholder files for parts 1 and 2

### Working on a Solution (Watch Mode)

The fastest way to iterate on a solution:

```bash
just work day-XX part1
```

This command starts a watch loop that:
- Monitors files in the day's directory
- Runs `cargo check` on every save
- Runs tests for the specified part
- Runs clippy linting

Example:
```bash
just work day-03 part1
```

### Running Tests

```bash
# Test a specific part
just test day-XX part1

# Test all parts for a day
cargo nextest run -p day-XX

# Test everything
cargo nextest run
```

### Running Solutions

```bash
# Run in release mode (optimized)
just run day-XX part1

# Run both parts
just run day-XX part1
just run day-XX part2
```

### Benchmarking

```bash
# Benchmark a specific part
just bench day-XX part1

# Benchmark all solutions
just bench-all
```

Benchmark results are saved to `day-XX.bench.txt` and `benchmarks.txt`.

### Linting

```bash
just lint day-XX
```

This runs `cargo clippy` with workspace configuration.

## Code Style

The project uses `rustfmt` with custom settings (see `rustfmt.toml`):

- **Max line width**: 60 characters (for readable diffs)
- **Function call width**: 40 characters
- **Comment width**: 80 characters
- **Import granularity**: Crate-level (groups imports by crate)

Format code with:
```bash
just format day-XX
```

### Solution Structure

Each day's solution follows this pattern:

```rust
// src/part1.rs or src/part2.rs
#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    // Parse input
    let data = parse_input(input)?;
    
    // Solve puzzle
    let result = solve(data);
    
    // Return result as string
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "example input from puzzle description";
        assert_eq!("expected_answer", process(input)?);
        Ok(())
    }
}
```

### Best Practices

1. **Write tests first**: Use the example from the puzzle description
2. **Use `miette::Result<T>`**: Provides beautiful error messages with context
3. **Add `#[tracing::instrument]`**: Enables structured logging for debugging
4. **Parse with `nom`**: For complex parsing, use parser combinators
5. **Leverage itertools**: Use `.chunks()`, `.combinations()`, `.windows()`, etc.
6. **Keep parts separate**: Don't refactor shared code until both parts are complete
7. **Profile before optimizing**: Use `just bench` to measure performance

## Available Just Commands

| Command | Description |
|---------|-------------|
| `just create day-XX` | Generate new day from template and fetch input |
| `just work day-XX part1` | Watch mode: auto-check, test, and lint on changes |
| `just test day-XX part1` | Run tests for specific part |
| `just run day-XX part1` | Run solution in release mode |
| `just bench day-XX part1` | Benchmark specific part |
| `just bench-all` | Benchmark all solutions |
| `just lint day-XX` | Run clippy on specific day |
| `just get-input day-XX` | Fetch puzzle input (requires SESSION in .env) |

## Tips and Tricks

### Input Parsing

- **Simple**: Use `lines()` and `split()` for basic parsing
- **Complex**: Use `nom` parser combinators for structured input
- **Example**:
  ```rust
  use nom::{
      bytes::complete::tag,
      character::complete::{digit1, newline},
      multi::separated_list1,
      IResult,
  };
  
  fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
      separated_list1(newline, nom::character::complete::u32)(input)
  }
  ```

### Error Handling

Use `miette` for beautiful error messages:

```rust
use miette::{miette, Result};

pub fn process(input: &str) -> Result<String> {
    let value = input
        .parse::<i32>()
        .map_err(|e| miette!("Failed to parse input: {}", e))?;
    
    Ok(value.to_string())
}
```

### Performance Optimization

1. **Profile first**: Use `just bench` to identify bottlenecks
2. **Consider parallelism**: Add `rayon` for embarrassingly parallel problems
3. **Use appropriate data structures**: `HashSet` for lookups, `Vec` for sequential access
4. **Avoid premature optimization**: Make it work, then make it fast

## Testing Strategy

1. **Example Tests**: Always test with the example from the puzzle description
2. **Edge Cases**: Empty input, single element, boundary values
3. **Integration Tests**: Test the full `process()` function
4. **Unit Tests**: Test helper functions separately

Example with `rstest`:

```rust
use rstest::rstest;

#[rstest]
#[case("input1", "expected1")]
#[case("input2", "expected2")]
fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
    assert_eq!(expected, process(input)?);
    Ok(())
}
```

## Build & CI

```bash
# Check compilation without building binaries
cargo check

# Build all workspace members (optimized)
cargo build --release

# Run all tests
cargo nextest run

# Run all benchmarks
cargo bench
```

## References

- [Advent of Code 2025](https://adventofcode.com/2025)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [nom Documentation](https://docs.rs/nom/)
- [itertools Documentation](https://docs.rs/itertools/)
- [miette Documentation](https://docs.rs/miette/)
- [divan Benchmarking Guide](https://docs.rs/divan/)

## License

This project is for educational purposes. Advent of Code is created by [Eric Wastl](http://was.tl/).

---

**Happy Coding! 🎄✨**
