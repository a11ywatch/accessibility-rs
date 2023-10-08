use super::tree::parse_accessibility_tree;
use accessibility_scraper::ElementRef;
use accessibility_scraper::Html;
use accessibility_tree::style::StyleSet;
use markup5ever::local_name;
use slotmap::DefaultKey;
use taffy::Taffy;

/// the intro to an audit
pub struct Auditor<'a> {
    /// the html document
    pub document: &'a Html,
    /// the tree to map to nodes
    pub tree: std::collections::BTreeMap<&'a str, Vec<(ElementRef<'a>, DefaultKey)>>,
    /// styles for the audit
    pub author: StyleSet,
    /// the matching context for css selectors
    pub match_context:
        selectors::matching::MatchingContext<'a, accessibility_scraper::selector::Simple>,
    /// layout handling
    pub taffy: Taffy,
}

impl<'a> Auditor<'a> {
    pub fn new(
        document: &'a Html,
        css_rules: &str,
        match_context: selectors::matching::MatchingContext<
            'a,
            accessibility_scraper::selector::Simple,
        >,
    ) -> Auditor<'a> {
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

        let (tree, taffy, match_context) =
            parse_accessibility_tree(&document, &author, match_context);

        Auditor {
            document,
            tree,
            author,
            match_context,
            taffy,
        }
    }
}
