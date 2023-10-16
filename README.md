# accessibility-rs

The Rust web accessibility engine.

## Usage

```toml
[dependencies]
accessibility-rs = "^0.0.30"
```

```rs
use accessibility_rs::{audit, AuditConfig};
// pass in raw html, optional css, bounding box clips, and locale for audit
let audit = accessibility_rs::audit(&AuditConfig::new(&html, &css, false, "en"));
```

### Documentation

[Module documentation with examples](https://docs.rs/accessibility-rs).

### Features

1. Accurate web accessibility WCAG audits.
1. Re-creating layout trees to get element positions without the DOM.
1. Ideal shapes for audits that scale.
1. Shortest path css selectors for elements.
1. Incredibly fast nanosecond audits.
1. i18n support.

### Contributing

To help improve the rules the following needs to be done:

1. Add the [rule](./RULES.md) to the tracking list - you can use the [standards list and mappings here](https://squizlabs.github.io/HTML_CodeSniffer/Standards/WCAG2/) for help.
1. Add the logic of handling the rule to [wcag_rule_map](./accessibility-rs/src/engine/rules/wcag_rule_map.rs) and the [techniques](./accessibility-rs/src/engine/rules/techniques.rs).
1. Add [unit](./accessibility-rs/tests/unit/mod.rs) test.

## Examples

1. [Wasm](https://webassembly.org/) example view [kayle_innate](https://github.com/a11ywatch/kayle/blob/main/kayle_innate/kayle_innate/src/lib.rs#L35).
1. Example integrating with a [headless browser](https://github.com/a11ywatch/kayle/blob/main/kayle/tests/innate.ts#L14).

### License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE_APACHE](LICENSE_APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE_MIT](LICENSE_MIT) or
   https://opensource.org/licenses/MIT)