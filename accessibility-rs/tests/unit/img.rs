//! Test for anchors.

use accessibility_rs::AuditConfig;

#[test]
/// img is missing an alt
fn _audit_img_missing_alt() {
    let audit = accessibility_rs::audit(AuditConfig::basic(
        r###"<html xmlns="http://www.w3.org/1999/xhtml" lang="en">
    <head>     
       <title>Missing Alt: Do not Use.</title>
    </head>   
    <body>     
    <img src="newsletter.gif" />
    </body> 
 </html>"###,
    ));
    let mut valid = true;

    for x in &audit {
        if x.code == "WCAGAAA.Principle1.Guideline1_1.H37" {
            valid = false;
            break;
        }
    }

    assert_eq!(valid, false)
}
