//! Test for anchors.

use accessibility_rs::AuditConfig;
use maud::html;


#[test]
/// anchor has valid contrast.
fn _audit_contrast_text_anchor() {
    let markup = html! {
        a href="www.example.com";
    };
    let audit = accessibility_rs::audit(AuditConfig::basic(
        markup
    ));
    let valid = !audit
        .iter()
        .any(|x| x.code == "WCAGAAA.Principle1.Guideline1_4.G18");

    assert_eq!(valid, false)
}
