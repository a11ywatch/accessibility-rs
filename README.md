# accessibility-rs

The Rust web accessibility engine.

```rs
// pass in html and css rules prior. If css rules are not passed in internal extraction is performed.
let audit = accessibility_rs::audit(&html, &css_rules);
```

## Features

1. Accurate web accessibility audits without a headless browser.
2. Re-creating layout trees to get element positions.
3. Ideal shapes for audits that scale.
4. Amazingly Fast audits.

## About

This crate is actively being developed. Some of the initial code is set as stubs until the feature is complete.
We have two params for the initial audit `html` and `css`. The reason we have css set is if coming from a browser
you can get all the stylesheets from the dom of external sheets at once. This saves time in re-gathering links later.

## Notes

Right now the Layout leaf parsing takes a good chunk of time. Until the parsing is done correctly this will be a crutch in performance
and main bottleneck. Removing the Clip handling will drastically shave off time from the runs.