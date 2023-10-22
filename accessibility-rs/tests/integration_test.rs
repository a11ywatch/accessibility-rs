//! Test suite for handling accessibility in Rust.

mod mocks;
use accessibility_rs::AuditConfig;
use mocks::mock;

#[test]
fn _audit() {
    let report = accessibility_rs::audit(AuditConfig::new(
        mock::MOCK_WEBSITE_HTML,
        &mock::MOCK_CSS_RULES,
        false,
        "en",
    ));
    println!("{:?}", report)
}
