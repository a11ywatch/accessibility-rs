# accessibility-rs

The Rust web accessibility engine.

## Usage

```toml
[dependencies]
accessibility-rs = "^0.0.26"
```

```rs
use accessibility_rs::{audit, AuditConfig};
// pass in raw html and css if coming from a headless browser
let audit = accessibility_rs::audit(&AuditConfig::new(&html, &css, false, "en"));
```

### License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE_APACHE](../LICENSE_APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE_MIT](../LICENSE_MIT) or
   https://opensource.org/licenses/MIT)