//! Test for label elements.
use accessibility_rs::AuditConfig;
use maud::html;

#[test]
/// label needs unique target ids
fn _audit_label_valid_name() {
    let markup = html! {
        label for="accessibility" { "My label" }
        input id="accessibility" type="text" placeholder="Accessibility rocks!" value="Here";
        label for="accessibility" { "My label" }
    };

    let audit = accessibility_rs::audit(AuditConfig::basic(&markup.into_string()));

    let valid = !audit
        .iter()
        .any(|x| x.code == "WCAGAAA.Principle1.Guideline1_3.H93");

    assert_eq!(valid, false)
}

#[test]
/// label has id that does not exist
fn _audit_label_id_noexist() {
    let markup = html! {
        label for="accessibility" { "My label" }
        input type="text" placeholder="Accessibility rocks!" value="Here";
    };

    let audit = accessibility_rs::audit(AuditConfig::basic(&markup.into_string()));

    let valid = !audit
        .iter()
        .any(|x| x.code == "WCAGAAA.Principle1.Guideline1_3.H44");

    assert_eq!(valid, false)
}
