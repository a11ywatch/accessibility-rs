use crate::engine::rules::rule::{Rule, Validation};
use crate::engine::rules::techniques::Techniques;
use crate::engine::rules::wcag_base::{Criteria, Guideline, Principle};
use crate::ElementRef;
use accessibility_scraper::Selector;
use selectors::Element;
use slotmap::DefaultKey;
use std::collections::BTreeMap;

/// a valid alt attribute for image
fn has_alt(ele: ElementRef<'_>) -> bool {
    let mut valid = true;
    match ele.attr("role") {
        Some(role) => {
            if role == "presentation" {
                return valid;
            }
        }
        _ => (),
    };
    match ele.attr("alt") {
        Some(_) => (),
        _ => valid = false,
    }
    valid
}

/// elements empty
fn is_empty(nodes: &Vec<(ElementRef<'_>, Option<DefaultKey>)>) -> bool {
    let mut empty = false;
    for ele in nodes {
        let ele = ele.0;
        empty = ele.inner_html().trim().is_empty();
    }
    empty
}

// todo: validate each element and add a shape that can prevent repitiion
lazy_static! {
    /// a list of rules that should be applied for WCAG1
    pub static ref RULES_A: BTreeMap<&'static str, Vec<Rule>> =
        vec![
            ("html", Vec::from([
                Rule::new(Techniques::H57, Criteria::Error, Principle::Understandable, Guideline::Readable, |_rule, nodes| {
                    Validation::new_issue(!nodes[0].0.attr("lang").unwrap_or_default().is_empty(), "2")
                }),
                Rule::new(Techniques::H57, Criteria::Error, Principle::Understandable, Guideline::Readable, |_rule, nodes| {
                    let lang = nodes[0].0.attr("lang").unwrap_or_default();
                    let alphabetic = lang.chars().all(|x| x.is_alphabetic());
                    // <https://www.rfc-editor.org/rfc/bcp/bcp47.txt>
                    Validation::new_issue(if lang.len() > 3 {
                        let mut c = lang.chars();
                        let has_underscore = c.nth(3).unwrap_or_default() == '_' || lang.len() >= 4 && c.nth(1).unwrap_or_default() == '_';
                        alphabetic && has_underscore && lang.len() < 12
                    } else {
                        alphabetic && lang.len() < 12
                    }, "3.Lang")
                }),
                Rule::new(Techniques::H57, Criteria::Error, Principle::Understandable, Guideline::Readable, |_rule, nodes| {
                    let lang = nodes[0].0.attr("xml:lang").unwrap_or_default();
                    let alphabetic = lang.chars().all(|x| x.is_alphabetic());
                   // <https://www.rfc-editor.org/rfc/bcp/bcp47.txt>
                   Validation::new_issue(if lang.len() > 3 {
                    let mut c = lang.chars();
                    let has_underscore = c.nth(3).unwrap_or_default() == '_' || lang.len() >= 4 && c.nth(1).unwrap_or_default() == '_';
                    alphabetic && has_underscore && lang.len() < 12
                    } else {
                        alphabetic && lang.len() < 12
                    }, "3.XmlLang")
                }),
            ])),
            ("meta", Vec::from([
                Rule::new(Techniques::F40, Criteria::Error, Principle::Operable, Guideline::EnoughTime, |_rule, nodes| {
                    let mut valid = true;

                    for node in nodes {
                        let element = node.0;
                        let meta_refresh = element.attr("http-equiv").unwrap_or_default();
                        if meta_refresh == "refresh" {
                            let content = element.attr("content").unwrap_or_default();
                            if content.contains("url") {
                                valid = content.starts_with("0;");
                            }
                        }
                    }

                    Validation::new_issue(valid, "2")
                }),
                Rule::new(Techniques::F41, Criteria::Error, Principle::Understandable, Guideline::EnoughTime, |_rule, nodes| {
                    let mut valid = true;

                    for node in nodes {
                        let element = node.0;
                        let meta_refresh = element.attr("http-equiv").unwrap_or_default();
                        if meta_refresh == "refresh" {
                            let content = element.attr("content").unwrap_or_default();
                            if !content.is_empty() {
                                valid = content == "0";
                            }
                        }
                    }

                    Validation::new_issue(valid, "2")
                }),
            ])),
            ("title", Vec::from([
                Rule::new(Techniques::H25, Criteria::Error, Principle::Operable, Guideline::Navigable, |_rule, nodes| {
                    Validation::new_issue(!nodes.is_empty(), "1.NoTitleEl")
                }),
                Rule::new(Techniques::H25, Criteria::Error, Principle::Operable, Guideline::Navigable, |_rule, nodes| {
                    Validation::new_issue(nodes.is_empty() || nodes[0].0.html().is_empty(), "1.EmptyTitle")
                }),
            ])),
            ("blink", Vec::from([
                Rule::new(Techniques::F47, Criteria::Error, Principle::Operable, Guideline::EnoughTime, |_rule, nodes| {
                    Validation::new_issue(nodes.is_empty(), "")
                }),
            ])),
            ("iframe", Vec::from([
                Rule::new(Techniques::H64, Criteria::Error, Principle::Operable, Guideline::Navigable, |_rule, nodes| {
                   Validation::new_issue(nodes.iter().all(|e| !e.0.attr("title").unwrap_or_default().is_empty()), "")
                }),
            ])),
            ("frame", Vec::from([
                Rule::new(Techniques::H64, Criteria::Error, Principle::Operable, Guideline::Navigable, |_rule, nodes| {
                    Validation::new_issue(nodes.iter().all(|e| !e.0.attr("title").unwrap_or_default().is_empty()), "")
                }),
            ])),
            ("form", Vec::from([
                Rule::new(Techniques::H32, Criteria::Error, Principle::Operable, Guideline::Predictable, |_rule, nodes| {
                    // check the first element for now
                    let mut valid = false;
                    let selector = unsafe { Selector::parse("button[type=submit]").unwrap_unchecked() };

                    for ele in nodes {
                        let ele = ele.0;
                        valid = match ele.select(&selector).next() {
                            Some(_) => true,
                            _ => false
                        };
                    }

                    Validation::new_issue(valid, "2")
                }),
                Rule::new(Techniques::H36, Criteria::Error, Principle::Perceivable, Guideline::TextAlternatives, |_rule, nodes| {
                    let mut valid = false;
                    let selector = unsafe { Selector::parse("input[type=image][name=submit]").unwrap_unchecked() };

                    for ele in nodes {
                        let ele = ele.0;
                        let mut elements = ele.select(&selector);

                        while let Some(el) = elements.next() {
                            valid = has_alt(el);
                        }
                    }

                    Validation::new_issue(valid, "")
                }),
            ])),
            ("a", Vec::from([
                Rule::new(Techniques::H30, Criteria::Error, Principle::Perceivable, Guideline::TextAlternatives, |_rule, nodes| {
                    let mut valid = true;
                    let selector = unsafe { Selector::parse("img").unwrap_unchecked() };
                    // todo: use tree to see if img exist to skip

                    for ele in nodes {
                        let ele = ele.0;
                        let mut elements = ele.select(&selector);

                        while let Some(el) = elements.next() {
                            valid = has_alt(el);
                        }
                    }

                    Validation::new_issue(valid, "2")
                }),
                Rule::new(Techniques::H91, Criteria::Error, Principle::Robust, Guideline::Compatible, |_rule, nodes| {
                    let mut valid = true;
                    for ele in nodes {
                        let ele = ele.0;
                        match ele.attr("href") {
                            Some(_) => {
                                valid = !ele.inner_html().trim().is_empty()
                            }
                            _ => ()
                        }
                    }
                    Validation::new_issue(valid, "A.NoContent")
                }),
                Rule::new(Techniques::H91, Criteria::Error, Principle::Robust, Guideline::Compatible, |_rule, nodes| {
                    let mut valid = true;
                    for ele in nodes {
                        let ele = ele.0;
                        valid = !ele.is_empty() || ele.has_attribute("id") || ele.has_attribute("href");
                    }
                    Validation::new_issue(valid, "A.EmptyNoId")
                }),
            ])),
            ("img", Vec::from([
                Rule::new(Techniques::H37, Criteria::Error, Principle::Perceivable, Guideline::TextAlternatives, |_rule, nodes| {
                    let mut valid = true;

                    for ele in nodes {
                        let ele = ele.0;
                        valid = has_alt(ele);
                    }

                    Validation::new_issue(valid, Techniques::H37.pairs()[0])
                }),
            ])),
            ("h1", Vec::from([
                Rule::new(Techniques::H42, Criteria::Error, Principle::Perceivable, Guideline::Adaptable, |_rule, nodes| {
                    Validation::new_issue(!is_empty(nodes), Techniques::H42.pairs()[0])
                }),
            ])),
            ("h2", Vec::from([
                Rule::new(Techniques::H42, Criteria::Error, Principle::Perceivable, Guideline::Adaptable, |_rule, nodes| {
                    Validation::new_issue(!is_empty(nodes), Techniques::H42.pairs()[0])
                }),
            ])),
            ("h3", Vec::from([
                Rule::new(Techniques::H42, Criteria::Error, Principle::Perceivable, Guideline::Adaptable, |_rule, nodes| {
                    Validation::new_issue(!is_empty(nodes), Techniques::H42.pairs()[0])
                }),
            ])),
            ("h4", Vec::from([
                Rule::new(Techniques::H42, Criteria::Error, Principle::Perceivable, Guideline::Adaptable, |_rule, nodes| {
                    Validation::new_issue(!is_empty(nodes), Techniques::H42.pairs()[0])
                }),
            ])),
            ("h5", Vec::from([
                Rule::new(Techniques::H42, Criteria::Error, Principle::Perceivable, Guideline::Adaptable, |_rule, nodes| {
                    Validation::new_issue(!is_empty(nodes), Techniques::H42.pairs()[0])
                }),
            ])),
            ("h6", Vec::from([
                Rule::new(Techniques::H42, Criteria::Error, Principle::Perceivable, Guideline::Adaptable, |_rule, nodes| {
                    Validation::new_issue(!is_empty(nodes), Techniques::H42.pairs()[0])
                }),
            ]))
        ]
        .into_iter()
        .collect();
}
