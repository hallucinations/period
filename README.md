# Tempus

A human-friendly time library for Rust.

## Usage

\```rust
use tempus::{today, yesterday, days_ago, weeks_from_now};

let today = today();
let yesterday = yesterday();
let three_days_ago = days_ago(3)?;
let next_week = weeks_from_now(2)?;
\```

## Functions

- `now()` / `today()` / `yesterday()` / `tomorrow()`
- `seconds/minutes/hours_ago(n)` / `seconds/minutes/hours_from_now(n)`
- `days/weeks/months/years_ago(n)` / `days/weeks/months/years_from_now(n)`
