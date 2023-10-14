use crate::engine::rules::rule::{Rule, Validation};
use crate::engine::rules::techniques::Techniques;
use crate::engine::rules::utils::nodes::{
    get_unique_selector, has_alt, is_empty, validate_missing_attr,
};
use crate::engine::rules::wcag_base::{Guideline, IssueType, Principle};
use accessibility_scraper::Selector;
use selectors::Element;
use std::collections::BTreeMap;

// todo: validate each element and add a shape that can prevent repitiion
lazy_static! {
    /// a list of rules that should be applied for WCAG1
    pub static ref RULES_A: BTreeMap<&'static str, Vec<Rule>> =
        vec![
            ("html", Vec::from([
                Rule::new(Techniques::H57, IssueType::Error, Principle::Understandable, Guideline::Readable, "1", |_rule, nodes| {
                    let n = nodes[0].0;
                    Validation::new_issue(!n.attr("lang").unwrap_or_default().is_empty() || !n.attr("xml:lang").unwrap_or_default().is_empty(), "2")
                }),
                Rule::new(Techniques::H57, IssueType::Error, Principle::Understandable, Guideline::Readable, "1", |_rule, nodes| {
                    let lang = nodes[0].0.attr("lang").unwrap_or_default();
                    let alphabetic = lang.chars().all(|x| x.is_alphabetic());
                    // <https://www.rfc-editor.org/rfc/bcp/bcp47.txt>
                    Validation::new_issue(if lang.len() > 3 {
                        let mut c = lang.chars();
                        let has_underscore = c.nth(2).unwrap_or_default() == '_' || lang.len() >= 4 && c.nth(1).unwrap_or_default() == '_';
                        alphabetic && has_underscore && lang.len() < 12
                    } else {
                        alphabetic && lang.len() < 12
                    }, "3.Lang")
                }),
                Rule::new(Techniques::H57, IssueType::Error, Principle::Understandable, Guideline::Readable, "1", |_rule, nodes| {
                    let lang = nodes[0].0.attr("xml:lang").unwrap_or_default();
                    let alphabetic = lang.chars().all(|x| x == '_' || x.is_alphabetic());
                   // <https://www.rfc-editor.org/rfc/bcp/bcp47.txt>
                   Validation::new_issue(if lang.len() > 3 {
                        let mut c = lang.chars();
                        let has_underscore = c.nth(2).unwrap_or_default() == '_' || lang.len() >= 4 && c.nth(1).unwrap_or_default() == '_';
                        alphabetic && has_underscore && lang.len() < 12
                    } else {
                        alphabetic && lang.len() < 12
                    }, "3.XmlLang")
                }),
            ])),
            ("meta", Vec::from([
                Rule::new(Techniques::F40, IssueType::Error, Principle::Operable, Guideline::EnoughTime, "1", |_rule, nodes| {
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
                Rule::new(Techniques::F41, IssueType::Error, Principle::Understandable, Guideline::EnoughTime, "1", |_rule, nodes| {
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
                Rule::new(Techniques::H25, IssueType::Error, Principle::Operable, Guideline::Navigable, "1", |_rule, nodes| {
                    Validation::new_issue(!nodes.is_empty(), "1.NoTitleEl")
                }),
                Rule::new(Techniques::H25, IssueType::Error, Principle::Operable, Guideline::Navigable, "1", |_rule, nodes| {
                    Validation::new_issue(nodes.is_empty() || nodes[0].0.html().is_empty(), "1.EmptyTitle")
                }),
            ])),
            ("blink", Vec::from([
                Rule::new(Techniques::F47, IssueType::Error, Principle::Operable, Guideline::EnoughTime, "2", |_rule, nodes| {
                    Validation::new_issue(nodes.is_empty(), "")
                }),
            ])),
            ("iframe", Vec::from([
                Rule::new(Techniques::H64, IssueType::Error, Principle::Operable, Guideline::Navigable, "1", |_rule, nodes| {
                    validate_missing_attr(nodes, "title", "1")
                }),
            ])),
            ("frame", Vec::from([
                Rule::new(Techniques::H64, IssueType::Error, Principle::Operable, Guideline::Navigable, "1", |_rule, nodes| {
                    validate_missing_attr(nodes, "title", "1")
                }),
            ])),
            ("form", Vec::from([
                Rule::new(Techniques::H32, IssueType::Error, Principle::Operable, Guideline::Predictable, "2", |_rule, nodes| {
                    let mut valid = false;
                    let mut elements = Vec::new();
                    let selector = unsafe { Selector::parse("button[type=submit]").unwrap_unchecked() };

                    for ele in nodes {
                        let ele = ele.0;
                        let e = ele.select(&selector);
                        let c = e.count();

                       if c == 1 {
                            valid = true;
                       } else {
                            valid = false;
                            elements.push(get_unique_selector(&ele))
                        }
                    }

                    Validation::new(valid, "2", elements, "")
                }),
                Rule::new(Techniques::H36, IssueType::Error, Principle::Perceivable, Guideline::TextAlternatives, "1", |_rule, nodes| {
                    let mut valid = false;
                    let mut elements = Vec::new();
                    let selector = unsafe { Selector::parse("input[type=image][name=submit]").unwrap_unchecked() };

                    for ele in nodes {
                        let ele = ele.0;
                        let mut e = ele.select(&selector);

                        while let Some(el) = e.next() {
                            let alt = has_alt(el);
                            if !alt {
                                elements.push(get_unique_selector(&ele))
                            }
                            valid = alt;
                        }
                    }

                    Validation::new(valid, "", elements, "")
                }),
            ])),
            ("a", Vec::from([
                Rule::new(Techniques::H30, IssueType::Error, Principle::Perceivable, Guideline::TextAlternatives, "1", |_rule, nodes| {
                    let mut valid = true;
                    let selector = unsafe { Selector::parse("img").unwrap_unchecked() };
                    let mut elements = Vec::new();

                    for ele in nodes {
                        let ele = ele.0;
                        let mut e = ele.select(&selector);

                        while let Some(el) = e.next() {
                            let alt = has_alt(el);
                            if !alt {
                                elements.push(get_unique_selector(&ele))
                            }
                            valid = alt;
                        }
                    }

                    Validation::new(valid, "2", elements, "")
                }),
                Rule::new(Techniques::H91, IssueType::Error, Principle::Robust, Guideline::Compatible, "2", |_rule, nodes| {
                    let mut valid = true;
                    let mut elements = Vec::new();

                    for ele in nodes {
                        let ele = ele.0;
                        match ele.attr("href") {
                            Some(_) => {
                                let empty = ele.inner_html().trim().is_empty();
                                if empty {
                                    elements.push(get_unique_selector(&ele))
                                }
                                valid = !empty
                            }
                            _ => ()
                        }
                    }
                    Validation::new(valid, "A.NoContent", elements, "")
                }),
                Rule::new(Techniques::H91, IssueType::Error, Principle::Robust, Guideline::Compatible, "2", |_rule, nodes| {
                    let mut valid = true;
                    let mut elements = Vec::new();
                    for ele in nodes {
                        let ele = ele.0;
                        let v = !ele.is_empty() || ele.has_attribute("id") || ele.has_attribute("href");
                        if !v {
                            elements.push(get_unique_selector(&ele))
                        }
                        valid = v;
                    }
                    Validation::new(valid, "A.EmptyNoId", elements, "")
                }),
            ])),
            ("img", Vec::from([
                Rule::new(Techniques::H37, IssueType::Error, Principle::Perceivable, Guideline::TextAlternatives, "1", |_rule, nodes| {
                    let mut valid = true;
                    let mut elements = Vec::new();

                    for ele in nodes {
                        let ele = ele.0;
                        let alt = has_alt(ele);
                        if !alt {
                            elements.push(get_unique_selector(&ele))
                        }
                        valid = alt;
                    }

                    Validation::new(valid, Techniques::H37.pairs()[0], elements, "")
                }),
            ])),
            ("h1", Vec::from([
                Rule::new(Techniques::H42, IssueType::Error, Principle::Perceivable, Guideline::Adaptable, "1", |_rule, nodes| {
                    Validation::new_issue(!is_empty(nodes), Techniques::H42.pairs()[0])
                }),
            ])),
            ("h2", Vec::from([
                Rule::new(Techniques::H42, IssueType::Error, Principle::Perceivable, Guideline::Adaptable, "1", |_rule, nodes| {
                    Validation::new_issue(!is_empty(nodes), Techniques::H42.pairs()[0])
                }),
            ])),
            ("h3", Vec::from([
                Rule::new(Techniques::H42, IssueType::Error, Principle::Perceivable, Guideline::Adaptable, "1", |_rule, nodes| {
                    Validation::new_issue(!is_empty(nodes), Techniques::H42.pairs()[0])
                }),
            ])),
            ("h4", Vec::from([
                Rule::new(Techniques::H42, IssueType::Error, Principle::Perceivable, Guideline::Adaptable, "1", |_rule, nodes| {
                    Validation::new_issue(!is_empty(nodes), Techniques::H42.pairs()[0])
                }),
            ])),
            ("h5", Vec::from([
                Rule::new(Techniques::H42, IssueType::Error, Principle::Perceivable, Guideline::Adaptable, "1", |_rule, nodes| {
                    Validation::new_issue(!is_empty(nodes), Techniques::H42.pairs()[0])
                }),
            ])),
            ("h6", Vec::from([
                Rule::new(Techniques::H42, IssueType::Error, Principle::Perceivable, Guideline::Adaptable, "1", |_rule, nodes| {
                    Validation::new_issue(!is_empty(nodes), Techniques::H42.pairs()[0])
                }),
            ]))
        ]
        .into_iter()
        .collect();
}
