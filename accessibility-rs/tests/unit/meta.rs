//! Test for meta data.

use accessibility_rs::AuditConfig;

use crate::mocks::mock;

#[test]
/// missing title element
fn _audit_missing_title() {
    let audit = accessibility_rs::audit(AuditConfig::basic(mock::MOCK_WEBSITE_HTML));
    let mut valid = true;

    for x in &audit {
        if x.code == "WCAGAAA.Principle2.Guideline2_4.H25" {
            valid = false;
            break;
        }
    }

    assert_eq!(valid, true)
}

#[test]
/// meta refresh redirect
fn _audit_meta_refresh() {
    let audit = accessibility_rs::audit(AuditConfig::basic(
        r###"<html xmlns="http://www.w3.org/1999/xhtml" lang="en">
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
    </html>"###,
    ));
    let mut valid = true;

    for x in &audit {
        if x.code == "WCAGAAA.Principle2.Guideline2_2.F40" {
            valid = false;
            break;
        }
    }

    assert_eq!(valid, false);

    let audit = accessibility_rs::audit(AuditConfig::basic(
        r###"<html xmlns="http://www.w3.org/1999/xhtml" lang="en">
<head>     
  <title>HTML Techniques for WCAG 2.0</title>     
  <meta http-equiv="refresh" content="60" />   
</head>   
<body>
</body> 
</html>"###,
    ));
    let mut valid = true;

    for x in &audit {
        if x.code == "WCAGAAA.Principle3.Guideline2_2.F41" {
            valid = false;
            break;
        }
    }

    assert_eq!(valid, false);
}

#[test]
/// no blink elements
fn _audit_blink_found() {
    let audit = accessibility_rs::audit(AuditConfig::basic(
        r###"<html xmlns="http://www.w3.org/1999/xhtml" lang="en">
<head>     
   <title>Do not use this!</title>      
</head>   
<body>     
<p>My Great Product <blink>Sale! $44,995!</blink></p>  
</body> 
</html>"###,
    ));
    let mut valid = true;

    for x in &audit {
        if x.code == "WCAGAAA.Principle2.Guideline2_2.F47" {
            valid = false;
            break;
        }
    }

    assert_eq!(valid, false);
}

#[test]
/// iframe missing title
fn _iframe_missing_title() {
    let audit = accessibility_rs::audit(AuditConfig {
        html: r###"<html xmlns="http://www.w3.org/1999/xhtml">
        <head>
          <title>A simple frameset document</title>
        </head>
        <frameset cols="10%, 90%">
          <frame src="nav.html" title="Main menu" />
          <frame src="doc.html" title="Documents" />
          <noframes>
            <body>
              <a href="lib.html" title="Library link">Select to
              go to the electronic library</a>
            </body>
          </noframes>
        </frameset>
      </html>"###
            .into(),
        ..Default::default()
    });
    let mut valid = true;

    for x in &audit {
        if x.code == "WCAGAAA.Principle2.Guideline2_4.H64" {
            valid = false;
            break;
        }
    }

    // this should be valid
    assert_eq!(valid, true);

    let config = AuditConfig::new(
        r###"<html xmlns="http://www.w3.org/1999/xhtml">
    <head>
      <title>A simple frameset document</title>
    </head>
    <frameset cols="10%, 90%">
      <frame src="nav.html" />
      <frame src="doc.html" id="doc" />
      <noframes>
        <body>
          <a href="lib.html" title="Library link">Select to
          go to the electronic library</a>
        </body>
      </noframes>
    </frameset>
  </html>"###,
        &"",
        false,
        "en",
    );
    let audit = accessibility_rs::audit(config);
    let mut valid = true;

    for x in &audit {
        if x.code == "WCAGAAA.Principle2.Guideline2_4.H64" {
            valid = false;
            break;
        }
    }

    assert_eq!(valid, false);
    let config = AuditConfig::new(
        r###" <html xmlns="http://www.w3.org/1999/xhtml">
    <head>
      <title>A document using iframe</title>
    </head>
  <iframe src="banner-ad.html" id="testiframe" 
    name="testiframe" title="Advertisement">
      <a href="banner-ad.html">Advertisement</a>
  </iframe>
  </html>"###,
        &"",
        false,
        "en",
    );

    let audit = accessibility_rs::audit(config);
    let mut valid = true;

    for x in &audit {
        if x.code == "WCAGAAA.Principle2.Guideline2_4.H64" {
            valid = false;
            break;
        }
    }

    assert_eq!(valid, true);
}

#[test]
/// incorrect xml:lang
fn _xml_lang_incorrect_format() {
    let audit = accessibility_rs::audit(AuditConfig::basic(
        r###"<html xmlns="http://www.w3.org/1999/xhtml" xml:lang="en_3">
<head>     
   <title>Do not use this!</title>      
</head>   
<body>     
</body> 
</html>"###,
    ));
    let mut valid = true;

    for x in &audit {
        if x.code == "WCAGAAA.Principle3.Guideline3_1.H57" {
            valid = false;
            break;
        }
    }

    assert_eq!(valid, false);
}
