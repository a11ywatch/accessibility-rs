//! Test for fieldsets.

use accessibility_rs::AuditConfig;
use maud::html;

#[test]
#[cfg(not(feature = "tokio"))]
/// anchor contains single img element without alt
fn _audit_missing_fieldset_legend() {
    let m = html! {
        fieldset {
            legend { "" };
            input type="radio" id="shakesp" name="hamlet" checked="checked" value="a";
            label for="shakesp" { "William Shakespeare" };
        }
    };

    let audit = accessibility_rs::audit(AuditConfig::basic(&m.into_string()));

    let valid = !audit
        .iter()
        .any(|x| x.code == "WCAGAAA.Principle1.Guideline1_3.H71");

    assert_eq!(valid, false)
}
