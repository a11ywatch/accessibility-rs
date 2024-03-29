use super::tree::parse_accessibility_tree;
use super::tree::parse_accessibility_tree_bounded;
use accessibility_scraper::ElementRef;
use accessibility_scraper::Html;
use accessibility_tree::style::StyleSet;
use markup5ever::local_name;
use taffy::TaffyTree;

/// The configuration for auditing
#[derive(Clone, Debug)]
pub struct Auditor<'a> {
    /// the html document
    pub document: &'a Html,
    /// the tree to map to nodes
    pub tree: std::collections::BTreeMap<&'a str, Vec<(ElementRef<'a>, Option<taffy::NodeId>)>>,
    /// styles for the audit
    pub author: StyleSet,
    /// language to get results in
    pub locale: &'a str,
}

impl<'a> Auditor<'a> {
    /// Create a new auditor that can be used to validate accessibility
    pub fn new(
        document: &'a Html,
        css_rules: &str,
        bounds: bool,
        locale: &'a str,
    ) -> (Auditor<'a>, Option<TaffyTree>) {
        // TODO: make stylesheet building optional and only on first requirement
        let author = {
            let mut author = accessibility_tree::style::StyleSetBuilder::new();
            if !css_rules.is_empty() {
                author.add_stylesheet(css_rules);
            } else {
                let selector =
                    unsafe { accessibility_scraper::Selector::parse("style").unwrap_unchecked() };
                let mut s = document.select(&selector);

                while let Some(node) = s.next() {
                    if let Some(type_attr) = node.attr(&local_name!("type")) {
                        if !type_attr.eq_ignore_ascii_case("text/css") {
                            continue;
                        }
                        author.add_stylesheet(&node.inner_html())
                    }
                }
            }
            author.finish()
        };

        let (tree, taffy) = if bounds {
            parse_accessibility_tree_bounded(&document, &author)
        } else {
            parse_accessibility_tree(&document, &author)
        };

        (
            Auditor {
                document,
                tree,
                author,
                locale,
            },
            taffy,
        )
    }
}
