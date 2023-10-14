//! Test generic html elements
use accessibility_rs::AuditConfig;

#[test]
/// duplicate html elements
fn _audit_duplicate_element_id() {
    let audit = accessibility_rs::audit(AuditConfig::basic(
        r###"<html lang="en">
    <head>     
       <title>Duplicate ID: Do not Use.</title>
    </head>   
    <body>     
        <div id="dog"></div>
        <div id="dog"></div>
    </body> 
 </html>"###,
    ));
    let mut valid = true;

    for x in &audit {
        if x.code == "WCAGAAA.Principle4.Guideline4_1.F77" {
            valid = false;
            break;
        }
    }

    assert_eq!(valid, false)
}
