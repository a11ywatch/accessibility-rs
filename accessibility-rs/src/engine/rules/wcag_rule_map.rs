use crate::engine::rules::rule::{Rule, Validation};
use crate::engine::rules::techniques::Techniques;
use crate::engine::rules::utils::nodes::{
    get_unique_selector, has_alt, has_alt_prop, has_prop, has_prop_value, validate_empty_nodes,
    validate_missing_attr,
};
use crate::engine::rules::wcag_base::{Guideline, IssueType, Principle};
use crate::i18n::locales::get_message_i18n_str_raw;
use accessibility_scraper::{ElementRef, Selector};
use selectors::Element;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::ops::Add;

lazy_static! {
    /// a list of rules that should be applied for WCAG2.0 A-AAA
    pub static ref RULES_A: BTreeMap<&'static str, Vec<Rule>> =
        vec![
            ("html", Vec::from([
                Rule::new(Techniques::H57.into(), IssueType::Error, Principle::Understandable, Guideline::Readable, "1", |nodes, _auditor| {
                    let n = nodes[0].0;
                    Validation::new_issue(!n.attr("lang").unwrap_or_default().is_empty() || !n.attr("xml:lang").unwrap_or_default().is_empty(), "2").into()
                }),
                Rule::new(Techniques::H57.into(), IssueType::Error, Principle::Understandable, Guideline::Readable, "1", |nodes, _auditor| {
                    let lang = nodes[0].0.attr("lang").unwrap_or_default();
                    let alphabetic = lang.chars().all(|x| x.is_alphabetic());
                    // <https://www.rfc-editor.org/rfc/bcp/bcp47.txt>
                    Validation::new_issue(if lang.len() > 3 {
                        let mut c = lang.chars();
                        let has_underscore = c.nth(2).unwrap_or_default() == '_' || lang.len() >= 4 && c.nth(1).unwrap_or_default() == '_';
                        alphabetic && has_underscore && lang.len() < 12
                    } else {
                        alphabetic && lang.len() < 12
                    }, "3.Lang").into()
                }),
                Rule::new(Techniques::H57.into(), IssueType::Error, Principle::Understandable, Guideline::Readable, "1", |nodes, _auditor| {
                    let lang = nodes[0].0.attr("xml:lang").unwrap_or_default();
                    let alphabetic = lang.chars().all(|x| x == '_' || x.is_alphabetic());
                   // <https://www.rfc-editor.org/rfc/bcp/bcp47.txt>
                   Validation::new_issue(if lang.len() > 3 {
                        let mut c = lang.chars();
                        let has_underscore = c.nth(2).unwrap_or_default() == '_' || lang.len() >= 4 && c.nth(1).unwrap_or_default() == '_';
                        alphabetic && has_underscore && lang.len() < 12
                    } else {
                        alphabetic && lang.len() < 12
                    }, "3.XmlLang").into()
                }),
                Rule::new(Techniques::H25.into(), IssueType::Error, Principle::Operable, Guideline::Navigable, "2", |nodes, _auditor| {
                    let selector = unsafe { Selector::parse("head > title").unwrap_unchecked() };

                    Validation::new_issue(nodes[0].0.select(&selector).count() >= 1, "1.NoTitleEl").into()
                })
            ])),
            ("meta", Vec::from([
                Rule::new(Techniques::F40.into(), IssueType::Error, Principle::Operable, Guideline::EnoughTime, "1", |nodes, _auditor| {
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

                    Validation::new_issue(valid, "2").into()
                }),
                Rule::new(Techniques::F41.into(), IssueType::Error, Principle::Understandable, Guideline::EnoughTime, "1", |nodes, _auditor| {
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

                    Validation::new_issue(valid, "2").into()
                }),
            ])),
            ("title", Vec::from([
                Rule::new(Techniques::H25.into(), IssueType::Error, Principle::Operable, Guideline::Navigable, "2", |nodes, _auditor| {
                    let mut valid = true;
                    for node in nodes {
                        let e = node.0.inner_html().is_empty();
                        if e {
                            valid = false;
                        }
                    }
                    Validation::new_issue(!nodes.is_empty() || valid, "1.EmptyTitle").into()
                }),
            ])),
            ("body", Vec::from([
                Rule::new(Techniques::G18.into(), IssueType::Error, Principle::Perceivable, Guideline::Distinguishable, "1", |nodes, auditor| {
                    use rgb::RGB8;
                    let mut validation_errors = Vec::new();

                    // todo: test for multiple contrast rules at once.
                    for node in nodes {
                        if node.0.has_children() {
                            let mut children = node.0.children();

                            while let Some(el) = children.next() {
                                match ElementRef::wrap(el) {
                                    Some(element) => {
                                        if vec![
                                            "h1",
                                            "h2",
                                            "h3",
                                            "h4",
                                            "h5",
                                            "h6",
                                            "a",
                                            "button",
                                            "p",
                                            "img",
                                            "span",
                                            "div",
                                            "li",
                                            "ol",
                                            "td",
                                            "th",
                                            "tr",
                                            "textarea",
                                            "select",
                                            "input"].contains(&element.value().name()) {
                                            let style = accessibility_tree::style::cascade::style_for_element_ref(
                                                &element,
                                                &auditor.author,
                                                &auditor.document
                                            );

                                            let font_size = style.font.font_size.0;
                                            let text_color = style.color.color;

                                            match element.parent() {
                                                Some(parent_node) => {
                                                    match ElementRef::wrap(parent_node) {
                                                        Some(parent_element) => {
                                                            let parent_style = accessibility_tree::style::cascade::style_for_element_ref(
                                                                &parent_element,
                                                                &auditor.author,
                                                                &auditor.document,
                                                            );

                                                            match parent_style.background.background_color {
                                                                cssparser::Color::RGBA(c) => {
                                                                    let parent_element_background_color = RGB8::from([c.red, c.green, c.blue]);
                                                                    let current_element_text_color = RGB8::from([text_color.red, text_color.green, text_color.blue]);
                                                                    let contrast_ratio = contrast::contrast::<_, f32>(parent_element_background_color, current_element_text_color);
                                                                    let min_contrast = if font_size.px <= 16.00 { 4.00 } else { 3.00 };

                                                                    if contrast_ratio <= min_contrast {
                                                                        let message =  t!(
                                                                            &get_message_i18n_str_raw(
                                                                                &Guideline::Distinguishable,
                                                                                "",
                                                                                "3_G18_or_G145.Fail",
                                                                                ""),
                                                                            locale = auditor.locale,
                                                                            required = min_contrast.to_string(),
                                                                            value = contrast_ratio.to_string());

                                                                        validation_errors.push(Validation::new_custom_issue(false, "", message).into())
                                                                    }
                                                                }
                                                                _ => ()
                                                            }
                                                        }
                                                        _ => ()
                                                    }
                                                }
                                                _ => ()
                                            }
                                        }
                                    }
                                    _ => ()
                                }
                            }
                        }
                    }

                    crate::engine::rules::rule::RuleValidation::Multi(validation_errors)
                }),
            ])),
            ("iframe", Vec::from([
                Rule::new(Techniques::H64.into(), IssueType::Error, Principle::Operable, Guideline::Navigable, "1", |nodes, _auditor| {
                    validate_missing_attr(nodes, "title", "1").into()
                }),
            ])),
            ("frame", Vec::from([
                Rule::new(Techniques::H64.into(), IssueType::Error, Principle::Operable, Guideline::Navigable, "1", |nodes, _auditor| {
                    validate_missing_attr(nodes, "title", "1").into()
                }),
            ])),
            ("form", Vec::from([
                Rule::new(Techniques::H32.into(), IssueType::Error, Principle::Operable, Guideline::Predictable, "2", |nodes, _auditor| {
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

                    Validation::new(valid, "2", elements, Default::default()).into()
                }),
                Rule::new(Techniques::H36.into(), IssueType::Error, Principle::Perceivable, Guideline::TextAlternatives, "1", |nodes, _auditor| {
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

                    Validation::new(valid, "", elements, Default::default()).into()
                }),
            ])),
            ("a", Vec::from([
                Rule::new(Techniques::H2.into(), IssueType::Error, Principle::Perceivable, Guideline::TextAlternatives, "1", |nodes, _auditor| {
                    let mut valid = true;
                    let selector = unsafe { Selector::parse("img").unwrap_unchecked() };
                    let mut elements = Vec::new();

                    for ele in nodes {
                        let ele = ele.0;
                        let mut e = ele.select(&selector);

                        while let Some(el) = e.next() {
                            let alt  = match el.attr("alt") {
                                Some(s) => s,
                                _ => "",
                            };

                            let text = ele.text().collect::<Vec<_>>().join("");
                            let text = text.trim();

                            if alt == text {
                                valid = false;
                                elements.push(get_unique_selector(&ele))
                            }
                        }
                    }

                    Validation::new(valid, "EG5", elements, Default::default()).into()
                }),
                Rule::new(Techniques::H30.into(), IssueType::Error, Principle::Perceivable, Guideline::TextAlternatives, "1", |nodes, _auditor| {
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

                    Validation::new(valid, "2", elements, Default::default()).into()
                }),
                Rule::new(Techniques::H91.into(), IssueType::Error, Principle::Robust, Guideline::Compatible, "2", |nodes, _auditor| {
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
                    Validation::new(valid, "A.NoContent", elements, Default::default()).into()
                }),
                Rule::new(Techniques::H91.into(), IssueType::Error, Principle::Robust, Guideline::Compatible, "2", |nodes, _auditor| {
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
                    Validation::new(valid, "A.EmptyNoId", elements, Default::default()).into()
                }),
            ])),
            ("img", Vec::from([
                Rule::new(Techniques::H37.into(), IssueType::Error, Principle::Perceivable, Guideline::TextAlternatives, "1", |nodes, _auditor| {
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

                    Validation::new(valid, "", elements, Default::default()).into()
                }),
                Rule::new(Techniques::H67.into(), IssueType::Error, Principle::Perceivable, Guideline::TextAlternatives, "1", |nodes, _auditor| {
                    let mut valid = true;
                    let mut elements = Vec::new();

                    for ele in nodes {
                        let ele = ele.0;
                        if has_prop(ele, "alt") && has_prop_value(ele, "title") {
                            valid = false;
                            elements.push(get_unique_selector(&ele))
                        }
                    }

                    Validation::new(valid, "1", elements, Default::default()).into()
                }),
            ])),
            ("h1", Vec::from([
                Rule::new(Techniques::H42.into(), IssueType::Error, Principle::Perceivable, Guideline::Adaptable, "1", |nodes, _auditor| {
                    validate_empty_nodes(nodes, "2").into()
                }),
            ])),
            ("h2", Vec::from([
                Rule::new(Techniques::H42.into(), IssueType::Error, Principle::Perceivable, Guideline::Adaptable, "1", |nodes, _auditor| {
                    validate_empty_nodes(nodes, "2").into()
                }),
            ])),
            ("h3", Vec::from([
                Rule::new(Techniques::H42.into(), IssueType::Error, Principle::Perceivable, Guideline::Adaptable, "1", |nodes, _auditor| {
                    validate_empty_nodes(nodes, "2").into()
                }),
            ])),
            ("h4", Vec::from([
                Rule::new(Techniques::H42.into(), IssueType::Error, Principle::Perceivable, Guideline::Adaptable, "1", |nodes, _auditor| {
                    validate_empty_nodes(nodes, "2").into()
                }),
            ])),
            ("h5", Vec::from([
                Rule::new(Techniques::H42.into(), IssueType::Error, Principle::Perceivable, Guideline::Adaptable, "1", |nodes, _auditor| {
                    validate_empty_nodes(nodes, "2").into()
                }),
            ])),
            ("h6", Vec::from([
                Rule::new(Techniques::H42.into(), IssueType::Error, Principle::Perceivable, Guideline::Adaptable, "1", |nodes, _auditor| {
                    validate_empty_nodes(nodes, "2").into()
                }),
            ])),
            ("label", Vec::from([
                Rule::new(Techniques::H93.into(), IssueType::Error, Principle::Perceivable, Guideline::Adaptable, "1", |nodes, _auditor| {
                    let mut valid = true;
                    let mut elements = Vec::new();
                    let mut id_map: HashMap<&str, u8> = HashMap::new();

                    for ele in nodes {
                        match ele.0.attr("for") {
                            Some(s) => {
                                if id_map.contains_key(s) {
                                    let u = id_map.get(s);
                                    match u {
                                        Some(u) => {
                                            valid = false;
                                            id_map.insert(s, u.add(1));
                                            elements.push(get_unique_selector(&ele.0))
                                        }
                                        _ => ()
                                    }
                                } else {
                                    id_map.insert(s, 1);
                                }
                            }
                            _ => ()
                        }
                    }

                    Validation::new(valid, "1", elements, Default::default()).into()
                }),
                Rule::new(Techniques::H44.into(), IssueType::Error, Principle::Perceivable, Guideline::Adaptable, "1", |nodes, _auditor| {
                    let mut valid = true;
                    let mut elements = Vec::new();

                    for ele in nodes {
                        let has_valid_aria_label = ele.0.attr("aria-label").map_or(false, |s| !s.trim().is_empty());
                        let mut has_valid_text_match = false;

                        if !has_valid_aria_label && ele.0.text().next().is_some() {
                            for child in ele.0.children() {
                                match ElementRef::wrap(child) {
                                    Some(child_element) => {
                                        let name = child_element.value().name();

                                        if vec!["textareas", "select"].contains(&name) {
                                            has_valid_text_match = true;
                                        } else if name == "input" {
                                            match child_element.attr("type") {
                                                Some(s) => {
                                                     if vec!["text", "file", "password"].contains(&s) {
                                                        has_valid_text_match = true;
                                                     }
                                                }
                                                _ => ()
                                            }
                                        }

                                        if has_valid_text_match {
                                            break;
                                        }
                                    }
                                    _ => ()
                                }
                            }
                        }

                        if !has_valid_aria_label && !has_valid_text_match {
                             match ele.0.attr("for") {
                                 Some(s) => {
                                     let selector = unsafe { Selector::parse(&("#".to_string() + &s)).unwrap_unchecked() };
                                     let root_tree = ele.0.tree().root();

                                     match ElementRef::new(root_tree) {
                                         t => {
                                             let e = t.select(&selector);

                                             if e.count() == 0 {
                                                 valid = false;
                                                 elements.push(get_unique_selector(&ele.0))
                                             }
                                         }
                                     }
                                 }
                                 _ => ()
                             }
                         }
                    }

                    Validation::new(valid, "NonExistent", elements, Default::default()).into()
                })
            ])),
            ("input", Vec::from([
                Rule::new(Techniques::H91.into(), IssueType::Error, Principle::Robust, Guideline::Compatible, "2", |nodes, auditor| {
                    let mut valid = true;
                    let mut elements = Vec::new();

                    for ele in nodes {
                        let ele = ele.0;
                        match ele.attr("type") {
                            Some(t) => {
                                if t == "submit" || t == "reset" || t == "button" {
                                    let is_valid = match ele.attr("value") {
                                        Some(_) => true,
                                        _ => false
                                    };

                                    if !is_valid {
                                        valid = false;
                                        elements.push(get_unique_selector(&ele))
                                    }
                                }
                            }
                            _ => ()
                        }
                    }

                    let message =  if !valid { t!(&get_message_i18n_str_raw( &Guideline::Compatible, "", "2_msg_pattern", ""), locale = auditor.locale, msgNodeType = r#""input""#, builtAttrs = r#""value""#) } else { Default::default() };

                    Validation::new(valid, "", elements, message).into()
                }),
                Rule::new(Techniques::H91.into(), IssueType::Error, Principle::Robust, Guideline::Compatible, "2", |nodes, auditor| {
                    let mut valid = true;
                    let mut elements = Vec::new();

                    for ele in nodes {
                        let ele = ele.0;
                        match ele.attr("type") {
                            Some(t) => {
                                if t == "submit" || t == "reset" || t == "button" {
                                    let is_valid = match ele.attr("value") {
                                        Some(v) => {
                                          if v.trim().is_empty() {
                                            false
                                          } else {
                                            true
                                          }
                                        }
                                        _ => false
                                    };

                                    if !is_valid {
                                        valid = false;
                                        elements.push(get_unique_selector(&ele))
                                    }
                                }
                            }
                            _ => ()
                        }
                    }

                    let message =  if !valid { t!(&get_message_i18n_str_raw( &Guideline::Compatible, "", "2_msg_pattern2", ""), locale = auditor.locale, msgNodeType = r#""input""#, builtAttrs = r#""value="something" ""#) } else { Default::default() };

                    Validation::new(valid, "", elements, message).into()
                }),
            ])),
            ("blink", Vec::from([
                Rule::new(Techniques::F47.into(), IssueType::Error, Principle::Operable, Guideline::EnoughTime, "2", |nodes, _auditor| {
                    Validation::new_issue(nodes.is_empty(), "").into()
                }),
            ])),
            ("object", Vec::from([
                Rule::new(Techniques::F47.into(), IssueType::Error, Principle::Perceivable, Guideline::TextAlternatives, "1", |nodes, _auditor| {
                    let mut valid = true;
                    let mut elements = Vec::new();

                    for ele in nodes {
                        let ele = ele.0;
                        let empty = ele.text();
                        if empty.count() >= 1 {
                            valid = false;
                            elements.push(get_unique_selector(&ele))
                        }
                    }

                    Validation::new(valid, "", elements, Default::default()).into()
                }),
            ])),
            ("area",Vec::from([
                Rule::new(Techniques::H24.into(), IssueType::Error, Principle::Perceivable, Guideline::TextAlternatives, "1", |nodes, _auditor| {
                    let mut valid = true;
                    let mut elements = Vec::new();

                    for ele in nodes {
                        let ele = ele.0;
                        if !has_alt_prop(ele) {
                            valid = false;
                            elements.push(get_unique_selector(&ele));
                        }
                    }

                    Validation::new(valid, "ImageMapAreaNoAlt", elements, Default::default()).into()
                })
            ])),
            ("map",Vec::from([
                Rule::new(Techniques::H24.into(), IssueType::Error, Principle::Perceivable, Guideline::TextAlternatives, "1", |nodes, _auditor|{
                    let mut valid = true;
                    let mut elements = Vec::new();

                    for ele in nodes{
                        let ele = ele.0;
                        if !has_alt_prop(ele){
                            valid =  false;
                            elements.push(get_unique_selector(&ele));
                        }
                    }

                    Validation::new(valid,"ImageMapNoAlt",elements, Default::default()).into()
                })
            ])),
            ("fieldset", Vec::from([
                Rule::new(Techniques::H71.into(), IssueType::Error, Principle::Perceivable, Guideline::Adaptable, "1", |nodes, _auditor| {
                    let mut valid = true;
                    let selector = unsafe { Selector::parse("legend").unwrap_unchecked() };
                    let mut elements = Vec::new();

                    for ele in nodes {
                        let ele = ele.0;
                        let mut e = ele.select(&selector);
                        let mut has_legend = false;

                        while let Some(el) = e.next() {
                            has_legend = true;
                            if el.text().count() == 0 {
                                valid = false;
                                elements.push(get_unique_selector(&ele))
                            }
                        }
                        if valid && !has_legend {
                            valid = false;
                        }
                    }

                    Validation::new(valid, "NoLegend", elements, Default::default()).into()
                }),
            ])),
            ("applet", Vec::from([
                Rule::new(Techniques::H35.into(), IssueType::Error, Principle::Perceivable, Guideline::TextAlternatives, "1", |nodes, _auditor| {
                    let mut valid = true;
                    let mut elements = Vec::new();

                    for ele in nodes {
                        let ele = ele.0;
                        if !has_alt_prop(ele) {
                            valid = false;
                            elements.push(get_unique_selector(&ele))
                        }
                    }

                    Validation::new(valid, "2", elements, Default::default()).into()
                }),
                Rule::new(Techniques::H35.into(), IssueType::Error, Principle::Perceivable, Guideline::TextAlternatives, "1", |nodes, _auditor| {
                    let mut valid = true;
                    let mut elements = Vec::new();

                    for ele in nodes {
                        let ele = ele.0;
                        let empty = ele.has_children() || !ele.inner_html().trim().is_empty();
                        if !empty {
                            valid = false;
                            elements.push(get_unique_selector(&ele))
                        }
                    }

                    Validation::new(valid, "3", elements, Default::default()).into()
                }),
            ])),
        ]
        .into_iter()
        .collect();
}
