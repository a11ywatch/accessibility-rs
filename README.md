# accessibility-rs

The Rust web accessibility engine.

## Usage

```toml
[dependencies]
accessibility-rs = "^0.1"
```

```rs
use accessibility_rs::{audit, AuditConfig};

fn main() {
  let html = r###"<html lang="en">
      <body>
          <a href="routes.html">
              <img src="topo.gif">
              Golf
          </a>
      </body>
  </html>"###;
  let css = "";
  // pass in raw html, optional css, bounding box clips, and locale for audit
  let audit = accessibility_rs::audit(&AuditConfig::new(&html, &css, false, "en"));
  println!("{:?}", audit);
}
```

With the Tokio runtime.

```toml
[dependencies]
accessibility-rs = { version = "^0.1", features = ["tokio"]}
```

```rs
use accessibility_rs::{audit, AuditConfig};
use tokio;

#[tokio::main]
async fn main() {
  let html = r###"<html lang="en">
      <body>
          <a href="routes.html">
              <img src="topo.gif">
              Golf
          </a>
      </body>
  </html>"###;
  let css = "";
  // pass in raw html, optional css, bounding box clips, and locale for audit
  let audit = accessibility_rs::audit(&AuditConfig::new(&html, &css, false, "en")).await;
  println!("{:?}", audit);
}
```

With the Spider full website crawling.

```toml
[dependencies]
accessibility-rs = { version = "^0.1", features = ["spider"]}
```

```rs
use accessibility_rs::{audit, AuditConfig};
use tokio;

#[tokio::main]
async fn main() {
  let audit = accessibility_rs::audit(&AuditConfig::new_website(&"https://choosealicense.com".into())).await;
  println!("{:?}", audit);
}
```

If you need to use concurrency use TendrilSink.

```rs
use accessibility_rs::{fast_html5ever, Auditor, Html};
use fast_html5ever::driver::{self, ParseOpts};
use tendril::TendrilSink;

let parser = driver::parse_document(
    Html::new_document(),
    ParseOpts::default(),
);
let document = parser.one("<html>MY html code </html>");

let auditor = Auditor::new(
    &document, &"", false, &"en",
);

let issues =
    accessibility_rs::engine::audit::wcag::WCAGAAA::audit(auditor)
        .await;
```

### Documentation

[Module documentation with examples](https://docs.rs/accessibility-rs).

### Features

1. Accurate web accessibility WCAG audits.
1. Incredibly fast nanosecond audits.
1. Ideal shapes for audits that scale.
1. Shortest path CSS selectors for elements.
1. i18n support for multiple languages.
1. Re-creating layout tree to get element position coordinates.
1. Crawling full websites lightning-fast using [spider](https://github.com/spider-rs/spider).
1. Low-level built to be used as an engine in browsers.

## [Benchmarks](./benches/)

```sh
audit-speed/core/audit: small html (4k iterations)
time: [55.689 µs 56.246 µs 57.110 µs]
audit-speed/core/audit: medium html (4k iterations)
time: [824.07 µs 830.30 µs 839.37 µs]
audit-speed/core/audit: large html (4k iterations)
time: [1.1206 ms 1.1260 ms 1.1321 ms]
audit-speed/core/audit: spider audit html (4k iterations)
time: [263.33 ms 266.55 ms 269.93 ms]
```

## Examples

1. [Wasm](https://webassembly.org/) example view [kayle_innate](https://github.com/a11ywatch/kayle/blob/main/kayle_innate/src/lib.rs#L18).
1. Example integrating with a [headless browser](https://github.com/a11ywatch/kayle/blob/main/kayle/tests/innate.ts#L14).

## Crate Features

1. [tokio](https://docs.rs/tokio/latest/tokio/): Enable tokio async runtime handling. Recommended for high freq server usage.
1. [rayon](https://docs.rs/rayon/latest/rayon/): Parallelism with rayon. (Expensive test future handling)
1. [rayon_wasm](https://lib.rs/crates/rayon-wasm): Enable the wasm runtime for rayon.
1. [spider](https://docs.rs/spider-rs/latest/spider/): Crawl entire websites using spider. Full website audits of 100-1,000 pages within milliseconds.

### Contributing

To help improve the rules the following needs to be done:

1. Add the [rule](./RULES.md) to the tracking list - you can use the [standards list and mappings here](https://squizlabs.github.io/HTML_CodeSniffer/Standards/WCAG2/) for help.
1. Add the logic of handling the rule to [wcag_rule_map](./accessibility-rs/src/engine/rules/wcag_rule_map.rs) and the [techniques](./accessibility-rs/src/engine/rules/techniques.rs).
1. Add [unit](./accessibility-rs/tests/unit/mod.rs) test.

### License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE_APACHE](LICENSE_APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE_MIT](LICENSE_MIT) or
  https://opensource.org/licenses/MIT)
