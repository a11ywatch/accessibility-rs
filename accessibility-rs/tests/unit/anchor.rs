//! Test for anchors.

use accessibility_rs::AuditConfig;

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
    let mut valid = true;

    for x in &audit {
        if x.code == "WCAGAAA.Principle1.Guideline1_1.H30" {
            valid = false;
            break;
        }
    }

    assert_eq!(valid, false)
}


#[test]
/// anchor contains valid href with no content
fn _audit_missing_anchor_content() {
    let audit = accessibility_rs::audit(AuditConfig::basic(
        r###"<html xmlns="http://www.w3.org/1999/xhtml" lang="en">
    <head>     
       <title>Missing Anchor Content Do not use!</title>
    </head>   
    <body>     
        <a href="www.example.com"></a>
    </body> 
 </html>"###,
    ));
    let mut valid = true;

    for x in &audit {
        if x.code == "WCAGAAA.Principle4.Guideline4_1.H91" {
            valid = false;
            break;
        }
    }

    assert_eq!(valid, false)
}
