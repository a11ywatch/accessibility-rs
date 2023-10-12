//! Test for headings.

use accessibility_rs::AuditConfig;

#[test]
/// empty headings
fn _audit_headings_empty() {
    let audit = accessibility_rs::audit(AuditConfig::basic(
        r###"<html xmlns="http://www.w3.org/1999/xhtml" lang="en">
    <head>     
       <title>Valid headings</title>
    </head>   
    <body>     
        <h1>Plant Foods that Humans Eat</h1>
        <p>There are an abundant number of plants that humans eat...</p>
        <h2>Fruit</h2>
        <p> A fruit is a structure of a plant that contains its
        seeds...</p>
        <h3>Apple</h3>
        <p>The apple is the pomaceous fruit of the apple tree...</p>
        <h3>Orange</h3>
        <p>The orange is a hybrid of ancient cultivated origin...</p>
        <h3>Banana</h3>
        <p>Banana is the common name for herbaceous plants ...</p>
        <h2>Vegetables</h2>
        <p>A vegetable is an edible plant or part of a plant other than a
        sweet fruit ...</p>
        <h3>Broccoli</h3>
        <p>Broccoli is a plant of the mustard/cabbage family ... </p>
        <h3>Brussels sprouts</h3>
        <p>The Brussels sprout of the Brassicaceae family, is a Cultivar
        group of wild cabbage ...</p>
        <h3>Green beans</h3>
        <p>Green beans have been bred for the fleshiness, flavor, or
        sweetness of their pods...</p>
    </body> 
 </html>"###,
    ));
    let mut valid = true;

    for x in &audit {
        if x.code == "WCAGAAA.Principle1.Guideline1_3.H42" {
            valid = false;
            break;
        }
    }

    assert_eq!(valid, true);

    let audit = accessibility_rs::audit(AuditConfig::basic(
        r###"<html xmlns="http://www.w3.org/1999/xhtml" lang="en">
    <head>     
       <title>Do not use missing Headings conent.</title>
    </head>   
    <body>     
        <h1> </h1>
        <p>There are an abundant number of plants that humans eat...</p>
        <h2> </h2>
        <p> A fruit is a structure of a plant that contains its
        seeds...</p>
        <h3> </h3>
        <p>The apple is the pomaceous fruit of the apple tree...</p>
        <h3></h3>
        <p>The orange is a hybrid of ancient cultivated origin...</p>
        <h3>   </h3>
        <p>Banana is the common name for herbaceous plants ...</p>
        <h2>   </h2>
        <p>A vegetable is an edible plant or part of a plant other than a
        sweet fruit ...</p>
        <h3>    </h3>
        <p>Broccoli is a plant of the mustard/cabbage family ... </p>
        <h3></h3>
        <p>The Brussels sprout of the Brassicaceae family, is a Cultivar
        group of wild cabbage ...</p>
        <h3>      </h3>
        <p>Green beans have been bred for the fleshiness, flavor, or
        sweetness of their pods...</p>
        <h4>  </h4>
        <h5>   </h5>
        <h6>   </h6>
    </body> 
 </html>"###,
    ));
    let mut valid = true;

    for x in &audit {
        if x.code == "WCAGAAA.Principle1.Guideline1_3.H42" {
            valid = false;
            break;
        }
    }

    assert_eq!(valid, false)
}
