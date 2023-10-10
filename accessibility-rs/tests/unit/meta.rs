//! Test for meta data.

use crate::mocks::mock;

#[test]
/// missing title element
fn _audit_missing_headers() {
    let audit = accessibility_rs::audit(mock::MOCK_WEBSITE_HTML, &"", false);
    let mut valid = true;

    for x in &audit {
        if x.code == "WCAGAAA.Principle3.Guideline3_2.H25" {
            valid = false;
            break;
        }
    }

    assert_eq!(valid, false)
}


#[test]
/// meta refresh redirect
fn _audit_meta_refresh() {
    let audit = accessibility_rs::audit(r###"<html xmlns="http://www.w3.org/1999/xhtml" lang="en">
    <head>     
       <title>Do not use this!</title>     
       <meta http-equiv="refresh"
       content="5; url=http://www.example.com/newpage" />   
    </head>   
    <body>     
       <p>       
          If your browser supports Refresh, you'll be       
          transported to our        
          <a href="http://www.example.com/newpage">new site</a>        
          in 5 seconds, otherwise, select the link manually.     
       </p>   
    </body> 
 </html>"###, &"", false);
    let mut valid = true;

    for x in &audit {
        if x.code == "WCAGAAA.Principle2.Guideline2_2.F40" {
            valid = false;
            break;
        }
    }

    assert_eq!(valid, false);

    let audit = accessibility_rs::audit(r###"<html xmlns="http://www.w3.org/1999/xhtml" lang="en">
    <head>     
      <title>HTML Techniques for WCAG 2.0</title>     
      <meta http-equiv="refresh" content="60" />   
    </head>   
    <body>
    </body> 
  </html>"###, &"", false);
    let mut valid = true;

    for x in &audit {
        if x.code == "WCAGAAA.Principle3.Guideline2_2.F41" {
            valid = false;
            break;
        }
    }

    assert_eq!(valid, false);
}

