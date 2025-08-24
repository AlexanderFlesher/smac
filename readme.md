# smac

## A small MAC address parsing library in no_std Rust

The `MacAddress` struct is six bytes, and implements `FromStr` and
`Display`. The `FromStr` implementation expects a string of six hex bytes
unseparated, or separated by space (' ') or colon (':'). The following
is a basic example of parsing a MAC address using the `ip` tool on Linux.

```rust
use once_cell::sync::Lazy;
use regex::Regex;
use subprocess::Exec;

use smac::{MacAddress, ParseError};

fn read_mac(interface: &str) -> Result<MacAddress, ParseError> {
    static MAC_PATTERN: Lazy<Regex> =
        Lazy::new(|| Regex::new("link/ether ((([a-fA-F0-9]){2}[:]){5}[a-fA-F0-9]{2})").unwrap());

    let command_result = match Exec::cmd("ip").args(&["link", "show", interface]).capture() {
        Ok(capture) => capture.stdout_str(),
        Err(_) => return Err(ParseError),
    };

    match MAC_PATTERN.captures(&command_result) {
        Some(matches) => matches[1].parse::<MacAddress>(),
        None => return Err(ParseError),
    }
}
```

`smac` uses `no_std` and only the unit tests depend on `alloc`.
