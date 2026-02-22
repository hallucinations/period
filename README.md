# Tempus

A human-friendly date and time library for Rust.

[![Rust](https://img.shields.io/badge/rust-1.93%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/vasanth/tempus/actions/workflows/ci.yml/badge.svg)](https://github.com/vasanth/tempus/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/tempus.svg)](https://crates.io/crates/tempus)

---

## Overview

Tempus provides an expressive, readable API for common date and time operations. Instead of wrestling with offsets and arithmetic, you write code that reads like English.

```rust
use tempus::{today, yesterday, days_ago, weeks_from_now, months_ago};

let today     = today();
let yesterday = yesterday();
let past      = days_ago(3)?;
let future    = weeks_from_now(2)?;
let earlier   = months_ago(6)?;
```

---

## Installation

Add Tempus to your `Cargo.toml`:

```toml
[dependencies]
tempus = "0.1"
```

---

## API Reference

### Current time

```rust
use tempus::{now, today};

let datetime = now();    // DateTime<Local>
let date     = today();  // NaiveDate
```

### Named days

```rust
use tempus::{yesterday, tomorrow};

let yesterday = yesterday(); // NaiveDate
let tomorrow  = tomorrow();  // NaiveDate
```

### Relative dates

All relative functions return `Result<NaiveDate, TempusError>` and reject negative values with a helpful error message.

```rust
use tempus::{days_ago, days_from_now, weeks_ago, weeks_from_now,
             months_ago, months_from_now, years_ago, years_from_now};

let d = days_ago(7)?;          // 7 days in the past
let d = days_from_now(30)?;    // 30 days in the future
let d = weeks_ago(2)?;         // 2 weeks in the past
let d = weeks_from_now(4)?;    // 4 weeks in the future
let d = months_ago(3)?;        // 3 calendar months in the past
let d = months_from_now(6)?;   // 6 calendar months in the future
let d = years_ago(1)?;         // 1 year in the past
let d = years_from_now(5)?;    // 5 years in the future
```

### Relative times

These return `Result<DateTime<Local>, TempusError>`.

```rust
use tempus::{seconds_ago, seconds_from_now, minutes_ago, minutes_from_now,
             hours_ago, hours_from_now};

let t = seconds_ago(30)?;        // 30 seconds in the past
let t = seconds_from_now(10)?;   // 10 seconds in the future
let t = minutes_ago(15)?;        // 15 minutes in the past
let t = minutes_from_now(45)?;   // 45 minutes in the future
let t = hours_ago(2)?;           // 2 hours in the past
let t = hours_from_now(8)?;      // 8 hours in the future
```

---

## Error handling

All fallible functions return `Result<T, TempusError>`. The error type is inspectable — you can match on specific variants:

```rust
use tempus::{days_ago, TempusError};

match days_ago(-5) {
    Err(TempusError::NegativeValue { suggestion, value, .. }) => {
        println!("Did you mean {}({})?", suggestion, value);
    }
    Err(TempusError::Overflow { unit, value }) => {
        println!("{} value {} is too large", unit, value);
    }
    Ok(date) => println!("{}", date),
}
```

Passing a negative value produces a descriptive error with a suggestion:

```
days must be positive. Did you mean days_from_now(5)?
```

`TempusError` implements both `std::fmt::Display` and `std::error::Error`, making it compatible with `?` in functions returning `Box<dyn Error>` or `anyhow::Error`.

---

## Design principles

- **Human-readable** — function names read like natural language
- **Explicit over implicit** — negative values are rejected with actionable error messages rather than silently producing unexpected results
- **Zero heap allocation on the error path** — error variants use `&'static str` fields
- **Composable** — `TempusError` implements `std::error::Error` for easy integration with the broader Rust ecosystem

---

## License

MIT
