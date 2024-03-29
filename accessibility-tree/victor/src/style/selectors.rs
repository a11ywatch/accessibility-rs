use crate::dom::{Document, Node, NodeId};
use crate::style::errors::RuleParseErrorKind;
use accessibility_scraper::selector::CssLocalName;
use accessibility_scraper::selector::Simple;
use fast_html5ever::{LocalName, Namespace};
use selectors::attr::{AttrSelectorOperation, CaseSensitivity, NamespaceConstraint};
use selectors::context::{MatchingContext, MatchingMode, QuirksMode};
use selectors::matching::{matches_selector, ElementSelectorFlags};

pub type SelectorList = selectors::SelectorList<Simple>;
pub type Selector = selectors::parser::Selector<Simple>;

pub fn matches(selector: &Selector, document: &Document, element: NodeId) -> bool {
    matches_selector(
        selector,
        0,
        None,
        &NodeRef {
            document,
            node: element,
        },
        &mut MatchingContext::new(MatchingMode::Normal, None, None, QuirksMode::NoQuirks),
        &mut |_, _| {},
    )
}

/// css parser
pub struct Parser;

impl<'i> selectors::parser::Parser<'i> for Parser {
    type Impl = Simple;
    type Error = RuleParseErrorKind<'i>;
}

#[derive(Copy, Clone)]
struct NodeRef<'a> {
    document: &'a Document,
    node: NodeId,
}

impl<'a> std::fmt::Debug for NodeRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.node.fmt(f)
    }
}

impl<'a> NodeRef<'a> {
    fn node(self) -> &'a Node {
        &self.document[self.node]
    }
    /// Returns the `Element` referenced by `self`.
    pub fn value(&self) -> &crate::dom::ElementData {
        self.node().as_element().unwrap()
    }
    /// Returns the value of an attribute.
    pub fn attr(&self, attr: &str) -> Option<&str> {
        self.value().get_attr(&LocalName::from(attr))
    }
    /// Returns if the element has the attibute and not empty
    pub fn has_attribute(&self, attr: &str) -> bool {
        match self.attr(attr) {
            Some(val) => !val.trim().is_empty(),
            None => false,
        }
    }
}

fn find_element<'a, F>(
    document: &'a Document,
    first: Option<NodeId>,
    next: F,
) -> Option<NodeRef<'a>>
where
    F: Fn(&Node) -> Option<NodeId>,
{
    let mut node = first?;
    loop {
        if document[node].as_element().is_some() {
            return Some(NodeRef { document, node });
        }
        node = next(&document[node])?
    }
}

impl<'a> selectors::Element for NodeRef<'a> {
    type Impl = Simple;

    #[inline]
    fn is_part(&self, _name: &LocalName) -> bool {
        false
    }

    fn opaque(&self) -> selectors::OpaqueElement {
        selectors::OpaqueElement::new::<Node>(self.node())
    }

    fn parent_element(&self) -> Option<Self> {
        let parent = self.node().parent?;
        self.document[parent].as_element()?;
        Some(NodeRef {
            document: self.document,
            node: parent,
        })
    }

    fn next_sibling_element(&self) -> Option<Self> {
        find_element(self.document, self.node().next_sibling, |node| {
            node.next_sibling
        })
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        find_element(self.document, self.node().previous_sibling, |node| {
            node.previous_sibling
        })
    }

    fn is_html_element_in_html_document(&self) -> bool {
        self.node().as_element().unwrap().name.ns == ns!(html) && self.node().in_html_document()
    }

    #[inline]
    fn imported_part(&self, _: &LocalName) -> Option<LocalName> {
        None
    }

    #[inline]
    fn exported_part(&self, _: &LocalName) -> Option<LocalName> {
        None
    }

    #[inline]
    fn is_same_type(&self, other: &Self) -> bool {
        self.value().name == other.value().name
    }

    #[inline]
    fn is_pseudo_element(&self) -> bool {
        false
    }

    #[inline]
    fn has_local_name(&self, name: &CssLocalName) -> bool {
        self.value().name.local == *name.0
    }

    #[inline]
    fn has_namespace(&self, namespace: &Namespace) -> bool {
        &self.value().name.ns == *&namespace
    }

    fn is_html_slot_element(&self) -> bool {
        false
    }

    fn parent_node_is_shadow_root(&self) -> bool {
        false
    }

    fn containing_shadow_host(&self) -> Option<Self> {
        None
    }

    fn attr_matches(
        &self,
        ns: &NamespaceConstraint<&Namespace>,
        local_name: &CssLocalName,
        operation: &AttrSelectorOperation<&accessibility_scraper::selector::CssString>,
    ) -> bool {
        self.node().as_element().unwrap().attrs.iter().any(|attr| {
            attr.name.local == *local_name.0
                && match *ns {
                    NamespaceConstraint::Any => true,
                    NamespaceConstraint::Specific(ns) => attr.name.ns == *ns,
                }
                && operation.eval_str(&attr.value)
        })
    }

    fn match_non_ts_pseudo_class<F>(
        &self,
        pseudo_class: &accessibility_scraper::selector::NonTSPseudoClass,
        _context: &mut MatchingContext<Self::Impl>,
        _flags_setter: &mut F,
    ) -> bool
    where
        F: FnMut(&Self, ElementSelectorFlags),
    {
        use accessibility_scraper::selector::NonTSPseudoClass::*;

        match *pseudo_class {
            Active | Focus | Hover | Enabled | Disabled | Checked | Indeterminate | Visited => {
                false
            }
            AnyLink | Link => {
                self.value().name.ns == ns!(html)
                    && matches!(
                        self.value().name.local,
                        local_name!("a") | local_name!("area") | local_name!("link")
                    )
                    && self.has_attribute("href")
            }
        }
    }

    fn match_pseudo_element(
        &self,
        pseudo_element: &accessibility_scraper::selector::PseudoElement,
        _context: &mut MatchingContext<Self::Impl>,
    ) -> bool {
        match *pseudo_element {}
    }

    fn is_link(&self) -> bool {
        let element = self.node().as_element().unwrap();
        element.name.ns == ns!(html)
            && matches!(
                element.name.local,
                local_name!("a") | local_name!("area") | local_name!("link")
            )
            && element.get_attr(&local_name!("href")).is_some()
    }

    fn has_id(&self, id: &CssLocalName, case_sensitivity: CaseSensitivity) -> bool {
        self.node()
            .as_element()
            .unwrap()
            .get_attr(&local_name!("id"))
            .map_or(false, |attr| {
                case_sensitivity.eq(id.0.as_bytes(), attr.as_bytes())
            })
    }

    fn has_class(&self, class: &CssLocalName, case_sensitivity: CaseSensitivity) -> bool {
        self.node()
            .as_element()
            .unwrap()
            .get_attr(&local_name!("class"))
            .map_or(false, |attr| {
                case_sensitivity.eq(class.0.as_bytes(), attr.as_bytes())
            })
    }

    fn is_empty(&self) -> bool {
        match self.node().first_child {
            None => true,
            Some(mut node) => loop {
                if self.document[node].as_element().is_some() {
                    return false;
                }
                if let Some(text) = self.document[node].as_text() {
                    if !text.is_empty() {
                        return false;
                    }
                }
                match self.document[node].next_sibling {
                    None => return true,
                    Some(n) => node = n,
                }
            },
        }
    }

    fn is_root(&self) -> bool {
        self.parent_element().is_none()
    }
}
