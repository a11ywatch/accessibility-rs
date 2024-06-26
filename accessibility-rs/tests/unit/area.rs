//! Test for anchors.

use accessibility_rs::AuditConfig;

#[test]
#[cfg(not(feature = "tokio"))]
fn _audit_missing_alt_area() {
    //alt attribute missing on first <area> tag
    let html = r#"
    <img src="workplace.jpg" alt="Workplace" usemap="\#workmap" width="400" height="379">
    <map name="workmap" alt="workplace image map" >
      <area shape="rect" coords="34,44,270,350"  href="computer.htm">
      <area shape="rect" coords="290,172,333,250" alt="Phone" href="phone.htm">
      <area shape="circle" coords="337,300,44" alt="Cup of coffee" href="coffee.htm">
    </map>"#;
    let audit: Vec<accessibility_rs::engine::issue::Issue> =
        accessibility_rs::audit(&AuditConfig::basic(html));

    let valid = audit
        .iter()
        .any(|x| x.message == "1_1_1_H24.ImageMapAreaNoAlt");

    assert_eq!(valid, true)
}

#[test]
#[cfg(not(feature = "tokio"))]
fn _audit_missing_alt_map() {
    // alt attribute missing on <map> tag
    let html = r#"
    <img src="workplace.jpg" alt="Workplace" usemap="\#workmap" width="400" height="379">
    <map name="workmap">
      <area shape="rect" coords="34,44,270,350" alt="Computer"  href="computer.htm">
      <area shape="rect" coords="290,172,333,250" alt="Phone" href="phone.htm">
      <area shape="circle" coords="337,300,44" alt="Cup of coffee" href="coffee.htm">
    </map>"#;
    let audit: Vec<accessibility_rs::engine::issue::Issue> =
        accessibility_rs::audit(&AuditConfig::basic(html));

    let valid = audit.iter().any(|x| x.message == "1_1_1_H24.ImageMapNoAlt");

    assert_eq!(valid, true)
}
