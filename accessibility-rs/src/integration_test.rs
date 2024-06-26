//! Test suite for handling accessibility in Rust.

mod mocks;
use accessibility_rs::AuditConfig;
use mocks::mock;

#[test]
fn _audit() {
    let report = accessibility_rs::audit(&AuditConfig::new(
        mock::MOCK_WEBSITE_HTML,
        &mock::MOCK_CSS_RULES,
        false,
        "en",
    ));
    println!("{:?}", report)
}

#[test]
fn _audit_large() {
    let report = accessibility_rs::audit(&AuditConfig::new(
        mock::MOCK_HTML_LARGE_PAGE,
        &mock::MOCK_CSS_RULES_LARGE,
        false,
        "en",
    ));
    println!("{:?}", report)
}

#[test]
fn _audit_bounded() {
    let report = accessibility_rs::audit(&AuditConfig::new(
        mock::MOCK_WEBSITE_HTML,
        &mock::MOCK_CSS_RULES,
        true,
        "en",
    ));
    println!("{:?}", report)
}

#[test]
fn _audit_large_bounded() {
    let report = accessibility_rs::audit(&AuditConfig::new(
        mock::MOCK_HTML_LARGE_PAGE,
        &mock::MOCK_CSS_RULES_LARGE,
        true,
        "en",
    ));
    println!("{:?}", report)
}
