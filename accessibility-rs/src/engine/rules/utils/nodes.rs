use crate::engine::rules::rule::Validation;
use crate::ElementRef;
use accessibility_scraper::Selector;
use selectors::Element;
use slotmap::DefaultKey;

type ElementNodes<'a> = Vec<(ElementRef<'a>, Option<DefaultKey>)>;

/// a valid alt attribute for image
pub fn has_alt(ele: ElementRef<'_>) -> bool {
    match ele.attr("role") {
        Some(role) => {
            if role == "presentation" {
                return true;
            }
        }
        _ => (),
    };
    has_alt_prop(ele)
}

/// a valid alt attribute for image
pub fn has_alt_prop(ele: ElementRef<'_>) -> bool {
    match ele.attr("alt") {
        Some(_) => true,
        _ => false,
    }
}

/// elements empty
pub fn is_empty(nodes: &ElementNodes) -> (bool, Vec<String>) {
    let mut valid = true;
    let mut elements = Vec::new();

    for ele in nodes {
        let ele = ele.0;
        let empty = ele.inner_html().trim().is_empty();
        if empty {
            valid = false;
            elements.push(get_unique_selector(&ele))
        }
    }

    (valid, elements)
}

/// elements empty with validation
pub fn validate_empty_nodes(nodes: &ElementNodes, id: &'static str) -> Validation {
    let (valid, elements) = is_empty(&nodes);
    Validation::new(valid, id, elements, Default::default())
}

/// check if the selector only exist for one element
pub fn single_selector(ele: &ElementRef<'_>, node_selector: &str) -> bool {
    match ele.tree().root().first_child() {
        Some(child) => match ElementRef::wrap(child) {
            Some(element) => match Selector::parse(node_selector) {
                Ok(s) => {
                    let e = element.select(&s);
                    e.count() == 1
                }
                _ => false,
            },
            _ => false,
        },
        _ => false,
    }
}

/// get the unique selector for an element
pub fn get_unique_selector(ele: &ElementRef<'_>) -> String {
    if ele.has_attribute("id") {
        "#".to_string() + ele.attr("id").unwrap_or_default()
    } else {
        let mut selector = String::new();
        let node_name = ele.value().name();

        if node_name == "body" || node_name == "html" {
            node_name.to_string()
        } else {
            let node_name = ele.value().name();

            if selector.is_empty() && ele.has_attribute("class") {
                let node_selector = node_name.to_string() + &ele.local_name().to_string();
                let only_selector = single_selector(ele, &node_selector);
                if only_selector {
                    selector = node_selector;
                }
            }

            if !selector.is_empty() {
                selector
            } else {
                if single_selector(ele, &node_name) {
                    node_name.to_string()
                } else {
                    let pos = get_sibling_position(ele);

                    let s = match ele.parent_element() {
                        Some(p) => {
                            if selector.is_empty() {
                                get_unique_selector(&p)
                            } else {
                                selector
                            }
                        }
                        _ => ele.value().name().to_string(),
                    };

                    s + ">:nth-child(" + &pos.to_string() + ")"
                }
            }
        }
    }
}

/// get sibling position of element
pub fn get_sibling_position(ele: &ElementRef<'_>) -> u8 {
    let mut i = 1;

    if ele.has_siblings() {
        let mut sibling = ele.prev_sibling();

        while let Some(e) = sibling {
            i += 1;
            sibling = e.prev_sibling();
        }
    }

    i
}

/// validate missing attribute
pub fn validate_missing_attr(
    nodes: &ElementNodes,
    attr: &'static str,
    id: &'static str,
) -> Validation {
    let mut elements = Vec::new();
    let mut valid = true;

    nodes.iter().for_each(|e| {
        if e.0.attr(attr).unwrap_or_default().is_empty() {
            valid = false;
            elements.push(get_unique_selector(&e.0))
        }
    });

    Validation::new(valid, id, elements, Default::default())
}
