//! Test for anchors.

use accessibility_rs::AuditConfig;
use maud::html;

#[test]
/// anchor contains single img element without alt
fn _audit_missing_alt_anchor_img() {
    let audit = accessibility_rs::audit(AuditConfig::basic(
        r###"<html xmlns="http://www.w3.org/1999/xhtml" lang="en">
    <head>     
       <title>Decrative Img: Do not use!</title>
    </head>   
    <body>     
        <a href="routes.html">
            <img src="topo.gif" /> 
        </a> 
    </body> 
 </html>"###,
    ));
    let valid = !audit
        .iter()
        .any(|x| x.code == "WCAGAAA.Principle1.Guideline1_1.H30");

    assert_eq!(valid, false)
}

#[test]
/// anchor contains valid href with no content
fn _audit_missing_anchor_content_valid_href() {
    let markup = html! {
        a href="www.example.com";
    };
    let audit = accessibility_rs::audit(AuditConfig::basic(&markup.into_string()));
    let valid = !audit
        .iter()
        .any(|x| x.code == "WCAGAAA.Principle4.Guideline4_1.H91");

    assert_eq!(valid, false)
}

#[test]
/// anchor is empty void
fn _audit_missing_anchor_content() {
    let markup = html! {
        a { "" }
    };
    let audit = accessibility_rs::audit(AuditConfig::basic(&markup.into_string()));
    let valid = !audit
        .iter()
        .any(|x| x.code == "WCAGAAA.Principle4.Guideline4_1.H91");

    assert_eq!(valid, false)
}
