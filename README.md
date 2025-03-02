# English to CronJob Syntax Converter
[![Crates.io](https://img.shields.io/crates/v/english-to-cron.svg)](https://crates.io/crates/english-to-cron)
[![Docs.rs](https://docs.rs/english-to-cron/badge.svg)](https://docs.rs/english-to-cron)

This project is inspired by the library natural-cron, which converts natural language into cron expressions. `english-to-cron` brings similar functionality to the Rust ecosystem, allowing developers to easily schedule cron jobs using English text.


## Features

- Converts various English text descriptions into cron job syntax.
- Supports complex patterns including specific days, time ranges, and more.
- Handles multiple time formats including AM/PM and 24-hour notation.

## Installation

Add the following line to your `Cargo.toml` under `[dependencies]`:

```toml
english_to_cron = "0.1" 
```

## Usage
Simply provide an English phrase describing the schedule, and the library will return the corresponding cron job syntax.
```rust
use english_to_cron::str_cron_syntax;

fn main() {
    assert_eq!(str_cron_syntax("every 15 seconds").unwrap(), "0/15 * * * * ? *");
    assert_eq!(str_cron_syntax("every minute").unwrap(), "0 * * * * ? *");
    assert_eq!(str_cron_syntax("every day at 4:00 pm").unwrap(), "0 0 16 */1 * ? *");
    assert_eq!(str_cron_syntax("at 10:00 am").unwrap(), "0 0 10 * * ? *");
    assert_eq!(str_cron_syntax("Run at midnight on the 1st and 15th of the month").unwrap(), "0 0 0 1,15 * ? *");
    assert_eq!(str_cron_syntax("on Sunday at 12:00").unwrap(), "0 0 12 ? * SUN *");
}
```

## Full List of Supported English Patterns

| English Phrase | CronJob Syntax |
|------------------------------------------------------------------	|---------------------------- |
| every 15 seconds | 0/15 * * * * ? * |
| run every minute | 0 * * * * ? * |
| fire every day at 4:00 pm | 0 0 16 */1 * ? * |
| at 10:00 am | 0 0 10 * * ? * |
| run at midnight on the 1st and 15th of the month | 0 0 0 1,15 * ? * |
| On Sunday at 12:00 | 0 0 12 ? * SUN * |
| 7pm every Thursday | 0 0 19 ? * THU * |
| midnight on Tuesdays | 0 0 ? * TUE * |


## Contributing
Contributions are welcome! Feel free to open issues or submit pull requests to help improve the library.