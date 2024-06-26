//! Test for applet.

use accessibility_rs::AuditConfig;

#[test]
#[cfg(not(feature = "tokio"))]
/// missing applet alt
fn _audit_missing_applet_alt() {
    let audit = accessibility_rs::audit(&AuditConfig::basic(
        r###"<html lang="en">
       <head>     
          <title>Missing applet alt: Do not use this!</title>
       </head>   
       <body>     
        <applet code="tictactoe.class" width="250" height="250">
                tic-tac-toe game
            </applet>    
       </body> 
    </html>"###,
    ));
    let mut valid = true;

    for x in &audit {
        if x.code == "WCAGAAA.Principle1.Guideline1_1.H35" {
            valid = false;
            break;
        }
    }

    assert_eq!(valid, false)
}

#[test]
#[cfg(not(feature = "tokio"))]
/// missing applet body
fn _audit_missing_applet_body() {
    let audit = accessibility_rs::audit(&AuditConfig::basic(
        r###"<html lang="en">
       <head>     
          <title>Missing applet alt: Do not use this!</title>
       </head>   
       <body>     
        <applet code="tictactoe.class" width="250" height="250" alt="tic tac toe"></applet>    
       </body> 
    </html>"###,
    ));
    let mut valid = true;

    for x in &audit {
        if x.code == "WCAGAAA.Principle1.Guideline1_1.H35" {
            valid = false;
            break;
        }
    }

    assert_eq!(valid, false)
}
