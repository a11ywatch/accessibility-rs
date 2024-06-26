//! Test suite for handling accessibility in Rust.

mod mocks;
use accessibility_rs::AuditConfig;
use mocks::mock;

#[cfg(feature = "tokio")]
#[tokio::test]
async fn _audit() {
    let report = accessibility_rs::audit(&AuditConfig::new(
        mock::MOCK_WEBSITE_HTML,
        &mock::MOCK_CSS_RULES,
        false,
        "en",
    ))
    .await;
    println!("{:?}", report)
}

#[test]
#[cfg(not(feature = "tokio"))]
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
#[cfg(not(feature = "tokio"))]
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
#[cfg(not(feature = "tokio"))]
fn _audit_with_layout() {
    let report = accessibility_rs::audit(&AuditConfig::new(
        mock::MOCK_WEBSITE_HTML,
        &mock::MOCK_CSS_RULES,
        true,
        "en",
    ));
    println!("{:?}", report)
}

#[test]
#[cfg(not(feature = "tokio"))]
fn _audit_large_with_layout() {
    let report = accessibility_rs::audit(&AuditConfig::new(
        mock::MOCK_HTML_LARGE_PAGE,
        &mock::MOCK_CSS_RULES_LARGE,
        true,
        "en",
    ));
    println!("{:?}", report)
}

#[test]
#[cfg(not(feature = "tokio"))]
fn _audit_xlarge() {
    let report = accessibility_rs::audit(&AuditConfig::new(
        mock::MOCK_WEBSITE_XLARGE_HTML,
        &mock::MOCK_CSS_RULES_XLARGE,
        false,
        "en",
    ));
    println!("{:?}", report)
}

#[tokio::test]
#[cfg(feature = "tokio")]
async fn _audit_xlarge() {
    let report = accessibility_rs::audit(&AuditConfig::new(
        mock::MOCK_WEBSITE_XLARGE_HTML,
        &mock::MOCK_CSS_RULES_XLARGE,
        false,
        "en",
    ))
    .await;
    println!("{:?}", report)
}

#[cfg(all(feature = "spider", not(feature = "rayon")))]
#[tokio::test]
async fn _audit_website() {
    let audit_config = AuditConfig::new_website("https://choosealicense.com", "", false, "");
    let report = accessibility_rs::audit(&audit_config).await;
    println!("{:?}", report)
}
