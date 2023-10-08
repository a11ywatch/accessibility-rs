//! Test suite for handling accessibility in Rust.

mod mock;

#[test]
fn _audit() {
    let _ = accessibility_rs::audit(mock::MOCK_WEBSITE_HTML, &mock::MOCK_CSS_RULES);
}