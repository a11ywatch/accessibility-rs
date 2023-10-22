//! Test for img elements.
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

#[test]
/// img is missing an alt
fn _audit_form_submit_img_missing_alt() {
    let audit = accessibility_rs::audit(AuditConfig::basic(
        r###"<html xmlns="http://www.w3.org/1999/xhtml" lang="en">
    <head>     
       <title>Missing Alt: Do not Use.</title>
    </head>   
    <body>     
        <form action="http://example.com/prog/text-read" method="post">
            <input type="image" name="submit" src="button.gif" />
        </form> 
        <form></form>   
    </body> 
 </html>"###,
    ));
    let mut valid = true;

    for x in &audit {
        if x.code == "WCAGAAA.Principle1.Guideline1_1.H36" {
            valid = false;
            break;
        }
    }

    assert_eq!(valid, false)
}

#[test]
/// img has empty alt and title
fn _audit_form_submit_img_has_alt_and_title() {
    let audit = accessibility_rs::audit(AuditConfig::basic(
        r###"<html xmlns="http://www.w3.org/1999/xhtml" lang="en">
    <head>     
       <title>Contains Title with Alt: Do not Use.</title>
    </head>   
    <body>     
        <img src="newsletter.gif" alt="" title="empty newsletter"/>
    </body> 
 </html>"###,
    ));
    let mut valid = true;

    for x in &audit {
        if x.code == "WCAGAAA.Principle1.Guideline1_1.H67" {
            valid = false;
            break;
        }
    }

    assert_eq!(valid, false)
}
