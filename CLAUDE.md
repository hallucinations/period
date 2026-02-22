# CLAUDE.md

This file provides context for AI assistants working in this repository.

## Project Overview

**Period** is a Rust library providing a human-friendly API for working with relative dates and times. Functions read like English (`days_ago(5)`, `hours_from_now(3)`) and wrap [chrono](https://docs.rs/chrono) with explicit error handling and zero-heap-allocation on the error path.

- **Crate:** `period` on crates.io
- **Version:** 0.1.1
- **Rust Edition:** 2024
- **MSRV:** 1.93
- **License:** MIT

## Motivation

The long-term goal is to give Ruby developers expressive, safe, and fast time handling — without Rails and without monkey-patching.

### Why Not Ruby's Stdlib?

Ruby ships three date/time classes (`Date`, `Time`, `DateTime`) with inconsistent, overlapping APIs:

- **Fragmentation** — `Date` is date-only; `Time` is date+time; `DateTime` is officially deprecated since Ruby 3.x but still widely used. No single unified type.
- **No relative helpers** — stdlib has no concept of `days.ago` or `weeks.from_now`. "3 days ago" requires `Date.today - 3` for dates but `Time.now - 3 * 24 * 60 * 60` for times.
- **No humanization** — no `time_ago_in_words` or "2 days ago" formatting.
- **No boundary helpers** — no `beginning_of_month`, `end_of_week`, `beginning_of_year`. Computing "start of this month" requires `Date.new(Date.today.year, Date.today.month, 1)`.
- **Verbose arithmetic** — month/year arithmetic is manual and breaks near year boundaries.
- **Timezone inconsistency** — `Time` supports timezones; `Date` is always naive; `DateTime` is deprecated. Local vs UTC ambiguity is a constant source of bugs.
- **No duration type** — no way to express "2 weeks" as a value and pass it around.

### Why Not Just Use ActiveSupport?

ActiveSupport solves most of the above, but:

- It is a Rails dependency — non-Rails Ruby projects pay a heavy cost to pull it in.
- It monkey-patches core classes globally (`2.days`, `3.hours`, `1.year` are added to `Integer`).
- The monkey-patching causes conflicts in gems and non-Rails codebases.
- It loads hundreds of kilobytes of code even if you only need time helpers.
- It obscures intent — `2.days.ago` looks like plain Ruby but is invisible magic with no explicit import.

`period` provides the same expressiveness as an explicit API, usable in any Ruby project.

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

Expose the library to Ruby using [magnus](https://github.com/matsadler/magnus). No monkey-patching; explicit imports; works in any Ruby project without Rails.

```ruby
require "period"

Period.now                    # => Time
Period.today                  # => Date
Period.yesterday              # => Date
Period.tomorrow               # => Date
Period.days_ago(3)            # => Date
Period.weeks_from_now(2)      # => Date
Period.months_ago(6)          # => Date
Period.hours_from_now(4)      # => Time
Period.humanize(some_time)    # => "2 hours ago"
Period.beginning_of_month     # => Date
Period.end_of_week            # => Date
Period.next_month             # => Date
d = Period::Duration.days(3)
Period.today + d              # => Date
Period.all_month              # => Range<Date>
Period.to_long_date(Date.today)       # => "February 21, 2026"
Period.parse("2026-02-21")    # => Date
Period.parse("next Monday")   # => Date
```

### Humanization

Convert a past or future datetime into a human-readable string. Mirrors ActiveSupport's `time_ago_in_words`.

```rust
pub fn humanize(datetime: DateTime<Local>) -> String
```

| Delta | Output |
|-------|--------|
| < 30 seconds | "just now" |
| < 90 seconds | "a minute ago" |
| < 45 minutes | "N minutes ago" |
| < 90 minutes | "an hour ago" |
| < 22 hours | "N hours ago" |
| < 36 hours | "yesterday" / "tomorrow" |
| < 25 days | "N days ago" |
| < 45 days | "a month ago" |
| < 10 months | "N months ago" |
| < 18 months | "a year ago" |
| otherwise | "N years ago" |

### Boundary Helpers

Start and end of calendar periods. Mirrors ActiveSupport's `beginning_of_*` / `end_of_*`.

```rust
// Day
beginning_of_day(datetime) -> DateTime<Local>
end_of_day(datetime) -> DateTime<Local>
noon(datetime) -> DateTime<Local>
midnight(datetime) -> DateTime<Local>

// Week (Monday default, configurable)
beginning_of_week(date) -> NaiveDate
end_of_week(date) -> NaiveDate

// Month
beginning_of_month(date) -> NaiveDate
end_of_month(date) -> NaiveDate

// Quarter
beginning_of_quarter(date) -> NaiveDate
end_of_quarter(date) -> NaiveDate

// Year
beginning_of_year(date) -> NaiveDate
end_of_year(date) -> NaiveDate
```

### Navigation Helpers

Move to adjacent calendar periods. Mirrors ActiveSupport's `prev_*` / `next_*`.

```rust
prev_day(date) -> NaiveDate
next_day(date) -> NaiveDate

prev_week(date) -> NaiveDate      // same weekday, previous week
next_week(date) -> NaiveDate

prev_month(date) -> NaiveDate     // same day of month, previous month
next_month(date) -> NaiveDate

prev_year(date) -> NaiveDate
next_year(date) -> NaiveDate
```

### Date Component Helpers

```rust
// Inspection
is_weekend(date) -> bool
is_weekday(date) -> bool
is_leap_year(date) -> bool

// Components
weekday(date) -> Weekday           // Monday, Tuesday…
day_of_week(date) -> u32           // 0 = Monday, 6 = Sunday
day_of_year(date) -> u32           // 1–366
days_in_month(date) -> u32         // 28–31
week_of_year(date) -> u32          // ISO 8601

// Constructing modified dates
with_day(date, day: u32) -> Result<NaiveDate, PeriodError>
with_month(date, month: u32) -> Result<NaiveDate, PeriodError>
with_year(date, year: i32) -> Result<NaiveDate, PeriodError>
```

### Duration Type

A first-class `Duration` value that can be stored, passed around, and applied to dates. Calendar-aware (supports months and years), distinct from `chrono::Duration`.

```rust
let d = Duration::seconds(30);
let d = Duration::minutes(15);
let d = Duration::hours(2);
let d = Duration::days(3);
let d = Duration::weeks(2);
let d = Duration::months(6);
let d = Duration::years(1);

// Arithmetic
date + d     -> Result<NaiveDate, PeriodError>
date - d     -> Result<NaiveDate, PeriodError>
datetime + d -> Result<DateTime<Local>, PeriodError>

// Inspection
d.as_seconds() -> Option<i64>   // None for calendar units
d.to_human()   -> String        // "3 days", "2 months"
```

### Range / Span Helpers

```rust
all_day(date)     -> Range<DateTime<Local>>
all_week(date)    -> Range<NaiveDate>
all_month(date)   -> Range<NaiveDate>
all_quarter(date) -> Range<NaiveDate>
all_year(date)    -> Range<NaiveDate>

contains(range, datetime) -> bool
overlaps(range_a, range_b) -> bool
duration_of(range) -> Duration
```

### Formatting Helpers

```rust
format_date(date, &str)    -> String   // strftime-style
format_time(datetime, &str) -> String
to_iso8601(datetime)       -> String   // "2026-02-21T14:30:00+00:00"
to_rfc2822(datetime)       -> String   // "Sat, 21 Feb 2026 14:30:00 +0000"
to_date_string(date)       -> String   // "2026-02-21"
to_long_date(date)         -> String   // "February 21, 2026"
to_short_date(date)        -> String   // "Feb 21, 2026"
```

### Parsing

```rust
parse_date(s: &str)     -> Result<NaiveDate, PeriodError>     // "2026-02-21", "21/02/2026", "Feb 21 2026"
parse_datetime(s: &str) -> Result<DateTime<Local>, PeriodError>  // ISO 8601, RFC 2822
parse_time(s: &str)     -> Result<NaiveTime, PeriodError>     // "14:30", "2:30pm"
```

### Future: Natural Language Parsing

```rust
parse_natural("3 days ago")          -> Result<NaiveDate, PeriodError>
parse_natural("next Monday")         -> Result<NaiveDate, PeriodError>
parse_natural("end of this month")   -> Result<NaiveDate, PeriodError>
parse_natural("in 2 weeks")          -> Result<NaiveDate, PeriodError>
parse_natural("last Friday")         -> Result<NaiveDate, PeriodError>
parse_natural("this time tomorrow")  -> Result<DateTime<Local>, PeriodError>
```
