use super::ElementRef;
use crate::selector::{CssLocalName, CssString, NonTSPseudoClass, PseudoElement, Simple};
use fast_html5ever::{LocalName, Namespace};
use selectors::{
    attr::{AttrSelectorOperation, CaseSensitivity, NamespaceConstraint},
    matching, Element, OpaqueElement,
};

/// Note: will never match against non-tree-structure pseudo-classes.
impl<'a> Element for ElementRef<'a> {
    type Impl = Simple;

    #[inline]
    fn is_part(&self, _name: &LocalName) -> bool {
        false
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

    fn opaque(&self) -> OpaqueElement {
        OpaqueElement::new(self.node.value())
    }

    fn parent_element(&self) -> Option<Self> {
        self.parent().and_then(ElementRef::wrap)
    }

    fn parent_node_is_shadow_root(&self) -> bool {
        false
    }

    fn containing_shadow_host(&self) -> Option<Self> {
        None
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        self.prev_siblings()
            .find(|sibling| sibling.value().is_element())
            .map(ElementRef::new)
    }

    fn next_sibling_element(&self) -> Option<Self> {
        self.next_siblings()
            .find(|sibling| sibling.value().is_element())
            .map(ElementRef::new)
    }

    fn is_html_element_in_html_document(&self) -> bool {
        // FIXME: Is there more to this?
        self.value().name.ns == ns!(html)
    }

    fn attr_matches(
        &self,
        ns: &NamespaceConstraint<&Namespace>,
        local_name: &CssLocalName,
        operation: &AttrSelectorOperation<&CssString>,
    ) -> bool {
        self.value().attrs.iter().any(|(key, value)| {
            !matches!(*ns, NamespaceConstraint::Specific(url) if *url != key.ns)
                && local_name.0 == key.local
                && operation.eval_str(value)
        })
    }

    fn match_non_ts_pseudo_class<F>(
        &self,
        pseudo_class: &NonTSPseudoClass,
        _context: &mut selectors::matching::MatchingContext<Self::Impl>,
        _flags_setter: &mut F,
    ) -> bool
    where
        F: FnMut(&Self, selectors::matching::ElementSelectorFlags),
    {
        use self::NonTSPseudoClass::*;

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
        _pe: &PseudoElement,
        _context: &mut matching::MatchingContext<Self::Impl>,
    ) -> bool {
        false
    }

    fn is_link(&self) -> bool {
        self.value().name() == "link"
    }

    fn is_html_slot_element(&self) -> bool {
        true
    }

    fn has_id(&self, id: &CssLocalName, case_sensitivity: CaseSensitivity) -> bool {
        match self.value().id() {
            Some(val) => case_sensitivity.eq(id.0.as_bytes(), val.as_bytes()),
            None => false,
        }
    }

    fn has_class(&self, name: &CssLocalName, case_sensitivity: CaseSensitivity) -> bool {
        self.value().has_class(&name.0, case_sensitivity)
    }

    fn is_empty(&self) -> bool {
        !self
            .children()
            .any(|child| child.value().is_element() || child.value().is_text())
    }

    fn is_root(&self) -> bool {
        self.parent()
            .map_or(false, |parent| parent.value().is_document())
    }
    // fn apply_selector_flags(&self, _flags: matching::ElementSelectorFlags) {}
}

#[cfg(test)]
mod tests {
    use crate::html::Html;
    use crate::selector::{CssLocalName, Selector};
    use selectors::attr::CaseSensitivity;
    use selectors::Element;

    #[test]
    fn test_has_id() {
        let html = "<p id='link_id_456'>hey there</p>";
        let fragment = Html::parse_fragment(html);
        let sel = Selector::parse("p").unwrap();

        let element = fragment.select(&sel).next().unwrap();
        assert!(element.has_id(
            &CssLocalName::from("link_id_456"),
            CaseSensitivity::CaseSensitive
        ));

        let html = "<p>hey there</p>";
        let fragment = Html::parse_fragment(html);
        let element = fragment.select(&sel).next().unwrap();
        assert!(!element.has_id(
            &CssLocalName::from("any_link_id"),
            CaseSensitivity::CaseSensitive
        ));
    }

    #[test]
    fn test_is_link() {
        let html = "<link href='https://www.example.com'>";
        let fragment = Html::parse_fragment(html);
        let sel = Selector::parse("link").unwrap();
        let element = fragment.select(&sel).next().unwrap();
        assert!(element.is_link());

        let html = "<p>hey there</p>";
        let fragment = Html::parse_fragment(html);
        let sel = Selector::parse("p").unwrap();
        let element = fragment.select(&sel).next().unwrap();
        assert!(!element.is_link());
    }

    #[test]
    fn test_has_class() {
        let html = "<p class='my_class'>hey there</p>";
        let fragment = Html::parse_fragment(html);
        let sel = Selector::parse("p").unwrap();
        let element = fragment.select(&sel).next().unwrap();
        assert!(element.has_class(
            &CssLocalName::from("my_class"),
            CaseSensitivity::CaseSensitive
        ));

        let html = "<p>hey there</p>";
        let fragment = Html::parse_fragment(html);
        let sel = Selector::parse("p").unwrap();
        let element = fragment.select(&sel).next().unwrap();
        assert!(!element.has_class(
            &CssLocalName::from("my_class"),
            CaseSensitivity::CaseSensitive
        ));
    }
}
