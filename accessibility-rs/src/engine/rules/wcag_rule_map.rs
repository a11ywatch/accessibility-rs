use crate::engine::rules::ids::Techniques;
use crate::engine::rules::rule::Rule;
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
                    (!nodes[0].0.attr("lang").unwrap_or_default().is_empty(), "2", Default::default())
                }),
                Rule::new(Techniques::H57, Criteria::Error, Principle::Understandable, Guideline::Readable, |_rule, nodes| {
                    let lang = nodes[0].0.attr("lang").unwrap_or_default();
                    // <https://www.rfc-editor.org/rfc/bcp/bcp47.txt>
                    (lang.chars().all(|x| x.is_alphanumeric()) && !lang.contains("_") && lang.len() < 12, "3.Lang", Default::default())
                }),
            ])),
           // empty titles
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

                    (valid, "2", Default::default())
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

                    (valid, "2", Default::default())
                }),
            ])),
            // empty titles
            ("title", Vec::from([
                Rule::new(Techniques::H25, Criteria::Error, Principle::Operable, Guideline::Navigable, |_rule, nodes| {
                    (!nodes.is_empty(), "1.NoTitleEl", Default::default())
                }),
                Rule::new(Techniques::H25, Criteria::Error, Principle::Understandable, Guideline::Predictable, |_rule, nodes| {
                    (nodes.is_empty() || nodes[0].0.html().is_empty(), "1.EmptyTitle", Default::default())
                }),
            ])),
            // missing form submit
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

                    (valid, "2", Default::default())
                }),
            ]))
        ]
        .into_iter()
        .collect();
}
