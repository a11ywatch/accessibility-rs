# accessibility-rs

The Rust web accessibility engine.

```rs
let audit = accessibility_rs::audit(&AuditConfig::new(&html, &css, false, "en"));
```

### Documentation

[Module documentation with examples](https://docs.rs/accessibility-rs).

### Features

1. Accurate web accessibility WCAG audits.
2. Re-creating layout trees to get element positions without the DOM.
3. Ideal shapes for audits that scale.
4. Incredibly fast audits.
5. Internationalization support.

### Contributing

To help improve the rules the following needs to be done:

1. Add the [rule](./RULES.md) to the tracking list.
1. Add the logic of handling the rule to [wcag_rule_map](./accessibility-rs/src/engine/rules/wcag_rule_map.rs) and the [techniques](./accessibility-rs/src/engine/rules/techniques.rs).
1. Add a unit test.

### License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE_APACHE](LICENSE_APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE_MIT](LICENSE_MIT) or
   https://opensource.org/licenses/MIT)