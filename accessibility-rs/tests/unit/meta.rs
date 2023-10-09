//! Test for meta data.

use crate::mocks::mock;

#[test]
/// missing title element
fn _audit_missing_headers() {
    let audit = accessibility_rs::audit(mock::MOCK_WEBSITE_HTML, &mock::MOCK_CSS_RULES, false);
    let mut valid = true;

    for x in &audit {
        if x.code == "WCAGAAA.Principle3.Guideline3_2.H25" {
            valid = false;
            break;
        }
    }

    assert_eq!(valid, false)
}
