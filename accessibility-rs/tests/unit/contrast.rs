//! Test for contrast.

use accessibility_rs::AuditConfig;
use maud::html;

#[test]
/// anchor has valid contrast.
fn _audit_contrast_text_anchor() {
    let markup = html! {
        body style="background: blue;" {
            a href="www.example.com" style="color:blue;" { "example" };
        }
    };
    let audit = accessibility_rs::audit(AuditConfig::basic(&markup.into_string()));
    let valid = !audit
        .iter()
        .any(|x| x.code == "WCAGAAA.Principle1.Guideline1_4.G18");

    assert_eq!(valid, false)
}
