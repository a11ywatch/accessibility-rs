use crate::engine::rules::rule::{Rule, Validation};
use crate::engine::rules::techniques::Techniques;
use crate::engine::rules::wcag_base::{Criteria, Guideline, Principle};
use accessibility_scraper::Selector;
use std::collections::BTreeMap;

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
                    // <https://www.rfc-editor.org/rfc/bcp/bcp47.txt>
                    Validation::new_issue(lang.chars().all(|x| x.is_alphanumeric()) && !lang.contains("_") && lang.len() < 12, "3.Lang")
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
            ])),
            ("a", Vec::from([
                Rule::new(Techniques::H30, Criteria::Error, Principle::Perceivable, Guideline::TextAlternatives, |_rule, nodes| {
                    // todo: use tree to see if img exist to skip
                    let mut valid = true;
                    let selector = unsafe { Selector::parse("img").unwrap_unchecked() };

                    for ele in nodes {
                        let ele = ele.0;
                        let mut elements = ele.select(&selector);

                        while let Some(el) = elements.next() {
                            // allow checking for role presentation not supported as wide as empty alt
                            match el.attr("role") {
                                Some(role) => {
                                    if role == "presentation" {
                                        continue;
                                    }
                                }
                                _ => ()
                            };
                            match el.attr("alt") {
                                Some(_) => (),
                                _ => valid = false
                            }
                        }

                    }

                    Validation::new_issue(valid, "2")
                }),
            ])),
            ("img", Vec::from([
                Rule::new(Techniques::H37, Criteria::Error, Principle::Perceivable, Guideline::TextAlternatives, |_rule, nodes| {
                    let mut valid = true;

                    for ele in nodes {
                        let ele = ele.0;
                        match ele.attr("role") {
                            Some(role) => {
                                if role == "presentation" {
                                    continue;
                                }
                            }
                            _ => ()
                        };
                        match ele.attr("alt") {
                            Some(_) => (),
                            _ => valid = false
                        }

                    }

                    Validation::new_issue(valid, Techniques::H37.pairs()[0])
                }),
            ]))
        ]
        .into_iter()
        .collect();
}
