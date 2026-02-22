# CLAUDE.md

This file provides context for AI assistants working in this repository.

## Project Overview

**Period** is a Rust library providing a human-friendly API for working with relative dates and times. Functions read like English (`days_ago(5)`, `hours_from_now(3)`) and wrap [chrono](https://docs.rs/chrono) with explicit error handling and zero-heap-allocation on the error path.

- **Crate:** `period` on crates.io
- **Version:** 0.1.1
- **Rust Edition:** 2024
- **MSRV:** 1.93
- **License:** MIT

## Repository Structure

```
period/
├── src/
│   ├── lib.rs        # Public re-exports only
│   ├── error.rs      # PeriodError enum and Display/Error impls
│   ├── now.rs        # now() and today() functions
│   └── relative.rs   # All relative date/time functions + tests
├── .github/
│   ├── workflows/
│   │   └── ci.yml                 # CI pipeline (test, lint, docs, MSRV)
│   └── copilot-instructions.md    # GitHub Copilot instructions (mirrors this file)
├── Cargo.toml
├── Cargo.lock        # Intentionally committed (library crate)
├── rust-toolchain.toml
└── README.md
```

## Development Commands

```bash
# Run all tests
cargo test --all-features

# Check formatting (do not auto-format in CI)
cargo fmt --check

# Auto-format locally
cargo fmt

# Lint (warnings are errors in CI)
cargo clippy -- -D warnings

# Generate docs (doc warnings are errors in CI)
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps

# Build
cargo build
cargo build --release

# Verify MSRV compatibility
cargo +1.93 check
```

## CI Pipeline

All checks run on push/PR to `main`. The pipeline must pass before merging:

| Job | Toolchain | Command |
|-----|-----------|---------|
| test | stable | `cargo test --all-features` |
| test (MSRV) | 1.93 | `cargo test --all-features` |
| lint | stable | `cargo fmt --check` + `cargo clippy -- -D warnings` |
| docs | 1.93 | `cargo doc --no-deps` (RUSTDOCFLAGS=-D warnings) |
| msrv | 1.93 | `cargo check` |

## Code Conventions

### Module Layout

- `lib.rs` contains only `pub use` re-exports — no logic.
- `error.rs` owns the `PeriodError` type.
- `now.rs` owns the non-fallible current-time functions.
- `relative.rs` owns all relative functions and their embedded unit tests.

### Function Design

Every relative function comes in a symmetrical pair:

```rust
pub fn days_ago(days: i64) -> Result<NaiveDate, PeriodError>
pub fn days_from_now(days: i64) -> Result<NaiveDate, PeriodError>
```

Rules for adding a new function:
1. Always add both the `_ago` and `_from_now` variants together.
2. Accept `i64` and reject negatives via `validate_non_negative()`.
3. Return `Result<_, PeriodError>` — do not panic.
4. Use `checked_*` arithmetic to detect overflow.
5. Mark with `#[inline]` and `#[must_use]`.
6. Add doc comments that include an `# Errors` section.

### Error Handling

`PeriodError` has two variants (marked `#[non_exhaustive]`):

```rust
NegativeValue { unit: &'static str, suggestion: &'static str, value: u64 }
Overflow      { unit: &'static str, value: i64 }
```

- Use `&'static str` for all fields — no heap allocation on error path.
- Error messages suggest the opposite function when a negative value is passed (e.g., "Did you mean `days_from_now(5)`?").
- Do not add new variants without a strong reason; `#[non_exhaustive]` exists precisely because the set may grow.

### Linting

The project enforces strict Clippy lints:

```toml
[lints.clippy]
all      = { level = "warn", priority = -1 }
pedantic = { level = "warn", priority = -1 }
module_name_repetitions = "allow"   # intentional exception
```

- Treat all Clippy warnings as errors when running locally before committing.
- `module_name_repetitions` is suppressed because function names like `PeriodError` intentionally repeat the crate name for clarity.

### Testing

Tests live inside `#[cfg(test)]` modules at the bottom of each source file — no separate `tests/` directory.

Every public function must have tests for:
1. Correct calculation (spot-check with a known offset).
2. Zero value (should return current date/time or a well-defined result).
3. Negative value → `PeriodError::NegativeValue`.
4. Overflow → `PeriodError::Overflow`.

Test naming: `test_<function_name>_<scenario>` (e.g., `test_days_ago_negative`, `test_days_ago_zero`).

### Documentation

- Every public item must have a doc comment.
- Include an `# Errors` section listing every `PeriodError` variant that can be returned.
- `cargo doc --no-deps` must pass with `RUSTDOCFLAGS="-D warnings"`.

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| chrono | 0.4 | Date/time primitives and arithmetic |

Keep the dependency count minimal. Avoid adding new dependencies unless they provide substantial value that cannot reasonably be implemented inline.

## Design Principles

1. **Human-readable API** — function names read like plain English.
2. **Explicit over implicit** — negative values are rejected with a helpful suggestion rather than silently negated.
3. **Zero heap allocation on error path** — all error fields are `&'static str` or primitive integers.
4. **Composable** — errors implement `std::error::Error` and integrate with the broader Rust error ecosystem.

## Roadmap

### Next: Ruby bindings via magnus

The immediate next goal is exposing this library to Ruby using [magnus](https://github.com/matsadler/magnus), enabling expressions like:

```ruby
Period.today             # => Date
Period.now               # => DateTime
Period.yesterday         # => Date
Period.tomorrow          # => Date
Period.days_ago(3)       # => Date
Period.weeks_from_now(2) # => Date
```

### Longer term

- **`humanize`** — convert a date/time to a human-readable string ("2 days ago", "just now", "last month")
- **`beginning_of` / `end_of`** — start and end of a week, month, or year
- **Natural language parsing** — `Period.parse("3 months from now")`
