# Period

A human-friendly date and time library for Rust.

[![Status: alpha](https://img.shields.io/badge/status-alpha-red.svg)](https://github.com/hallucinations/period)
[![Rust](https://img.shields.io/badge/rust-1.93%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/hallucinations/period/actions/workflows/ci.yml/badge.svg)](https://github.com/hallucinations/period/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/period.svg)](https://crates.io/crates/period)

---

> **Note:** Period is a learning project, built to explore the Rust ecosystem and the process of authoring Ruby extensions from Rust. Its public API is intentionally expressive, but the implementation is a thin layer over [chrono](https://docs.rs/chrono) with no additional performance guarantees, hardening, or production-level validation. If you need reliable, well-tested date and time handling in production, use `chrono` directly or a library with a longer track record. That said, if the API style appeals to you, you are welcome to use it — just do so with that context in mind.

---

## Overview

Period provides an expressive, readable API for common date and time operations. Instead of wrestling with offsets and arithmetic, you write code that reads like English.

Every relative function returns a `Relative` value — a thin wrapper around `DateTime<Local>` — which you convert to whichever representation you need:

```rust
use period::{today, yesterday, days_ago, weeks_from_now, months_ago, hours_ago, humanize, now};

let today     = today();                             // NaiveDate
let yesterday = yesterday();                         // NaiveDate
let past      = days_ago(3)?.as_date();              // NaiveDate
let future    = weeks_from_now(2)?.as_date();        // NaiveDate
let earlier   = months_ago(6)?.as_datetime();        // DateTime<Local>
let time_of_day = hours_ago(4)?.as_time();           // NaiveTime (time-of-day only)
let label     = humanize(now());                     // "just now"
```

---

## Installation

Add Period to your `Cargo.toml`:

```toml
[dependencies]
period = "0.1"
```

---

## API Reference

### Current time

```rust
use period::{now, today};

let datetime = now();    // DateTime<Local>
let date     = today();  // NaiveDate
```

### Named days

```rust
use period::{yesterday, tomorrow};

let yesterday = yesterday(); // NaiveDate
let tomorrow  = tomorrow();  // NaiveDate
```

### Relative values

All relative functions return `Result<Relative, PeriodError>`. Call `.as_date()`, `.as_datetime()`, or `.as_time()` on the result to extract the representation you need.

```rust
use period::{days_ago, days_from_now, weeks_ago, weeks_from_now,
             months_ago, months_from_now, years_ago, years_from_now,
             seconds_ago, seconds_from_now, minutes_ago, minutes_from_now,
             hours_ago, hours_from_now};

// Dates
let d = days_ago(7)?.as_date();           // NaiveDate — 7 days in the past
let d = days_from_now(30)?.as_date();     // NaiveDate — 30 days in the future
let d = weeks_ago(2)?.as_date();          // NaiveDate — 2 weeks in the past
let d = weeks_from_now(4)?.as_date();     // NaiveDate — 4 weeks in the future
let d = months_ago(3)?.as_date();         // NaiveDate — 3 calendar months in the past
let d = months_from_now(6)?.as_date();    // NaiveDate — 6 calendar months in the future
let d = years_ago(1)?.as_date();          // NaiveDate — 1 year in the past
let d = years_from_now(5)?.as_date();     // NaiveDate — 5 years in the future

// DateTimes
let t = seconds_ago(30)?.as_datetime();       // DateTime<Local> — 30 seconds ago
let t = seconds_from_now(10)?.as_datetime();  // DateTime<Local> — 10 seconds from now
let t = minutes_ago(15)?.as_datetime();       // DateTime<Local> — 15 minutes ago
let t = minutes_from_now(45)?.as_datetime();  // DateTime<Local> — 45 minutes from now
let t = hours_ago(2)?.as_datetime();          // DateTime<Local> — 2 hours ago
let t = hours_from_now(8)?.as_datetime();     // DateTime<Local> — 8 hours from now

// Time-of-day only
let t = hours_ago(1)?.as_time(); // NaiveTime
```

### The `Relative` type

`Relative` is a `Copy` wrapper around `DateTime<Local>` with three conversion methods:

| Method | Returns | Description |
|--------|---------|-------------|
| `.as_datetime()` | `DateTime<Local>` | Full date and time with timezone |
| `.as_date()` | `NaiveDate` | Calendar date only |
| `.as_time()` | `NaiveTime` | Time-of-day only |

`From<Relative>` is implemented for all three types, so you can use `.into()` or type inference in assignments:

```rust
use period::days_ago;
use chrono::NaiveDate;

let date: NaiveDate = days_ago(10)?.into();
```

### Humanize

Convert any `DateTime<Local>` into a human-readable relative string.

```rust
use period::humanize;
use chrono::Local;

let dt = Local::now() - chrono::Duration::minutes(35);
println!("{}", humanize(dt)); // "35 minutes ago"

let dt = Local::now() + chrono::Duration::hours(2);
println!("{}", humanize(dt)); // "in 2 hours"
```

You can also pass the inner datetime from a `Relative` value:

```rust
use period::{hours_ago, humanize};

let label = humanize(hours_ago(3)?.as_datetime()); // "3 hours ago"
```

| Absolute delta | Past              | Future           |
|----------------|-------------------|------------------|
| < 30 s         | `"just now"`      | `"just now"`     |
| < 90 s         | `"a minute ago"`  | `"in a minute"`  |
| < 45 min       | `"N minutes ago"` | `"in N minutes"` |
| < 90 min       | `"an hour ago"`   | `"in an hour"`   |
| < 22 h         | `"N hours ago"`   | `"in N hours"`   |
| < 36 h         | `"yesterday"`     | `"tomorrow"`     |
| < 25 days      | `"N days ago"`    | `"in N days"`    |
| < 45 days      | `"a month ago"`   | `"in a month"`   |
| < 10 months    | `"N months ago"`  | `"in N months"`  |
| < 18 months    | `"a year ago"`    | `"in a year"`    |
| ≥ 18 months    | `"N years ago"`   | `"in N years"`   |

> **Note:** The `< 90 s` bucket uses the article form (`"a minute ago"`). Values in
> the `< 45 min` bucket that round to `n = 1` produce `"1 minute ago"`, so the
> transition at 90 s is `"a minute ago"` → `"1 minute ago"` → `"2 minutes ago"`.
>
> **Note:** `"yesterday"` / `"tomorrow"` are triggered by elapsed seconds (22–35 h),
> not by calendar-day boundaries. Months are approximated as 30 days.

---

## Error handling

All fallible functions return `Result<_, PeriodError>`. The error type is inspectable — you can match on specific variants:

```rust
use period::{days_ago, PeriodError};

match days_ago(-5) {
    Err(PeriodError::NegativeValue { suggestion, value, .. }) => {
        println!("Did you mean {}({})?", suggestion, value);
    }
    Err(PeriodError::Overflow { unit, value }) => {
        println!("{} value {} is too large", unit, value);
    }
    Ok(relative) => println!("{}", relative.as_date()),
}
```

Passing a negative value produces a descriptive error with a suggestion:

```
days must be positive. Did you mean days_from_now(5)?
```

`PeriodError` implements both `std::fmt::Display` and `std::error::Error`, making it compatible with `?` in functions returning `Box<dyn Error>` or `anyhow::Error`.

---

## Design principles

- **Human-readable** — function names read like natural language
- **Explicit over implicit** — negative values are rejected with actionable error messages rather than silently producing unexpected results
- **Zero heap allocation on the error path** — error variants use `&'static str` fields
- **Composable** — `PeriodError` implements `std::error::Error` for easy integration with the broader Rust ecosystem

---

## License

MIT
