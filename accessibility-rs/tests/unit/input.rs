//! Test for input elements.
use accessibility_rs::AuditConfig;

#[test]
#[cfg(not(feature = "tokio"))]
/// input is missing a valid name
fn _audit_input_valid_name() {
    let audit = accessibility_rs::audit(&AuditConfig::basic(
        r###"<html lang="en">
    <head>     
       <title>Missing Form control name: Do not Use.</title>
    </head>   
    <body>     
        <input type="button" /> 
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
