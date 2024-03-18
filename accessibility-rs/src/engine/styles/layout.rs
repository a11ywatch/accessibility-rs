use accessibility_scraper::selector::Simple;
use accessibility_scraper::ElementRef;
use accessibility_scraper::Html;
use accessibility_tree::style::values::LengthOrPercentageOrAuto;
use accessibility_tree::style::ComputedValues;
use accessibility_tree::style::StyleSet;
use ego_tree::NodeRef;
use selectors::matching::MatchingContext;
use std::collections::HashSet;
use std::sync::Arc;
use taffy::prelude::*;
use taffy::style::Dimension;

lazy_static! {
    static ref NODE_IGNORE: HashSet<&'static str> =
        HashSet::from(["meta", "style", "link", "script", "head", "html", "body"]);
}

/// length to taffy dimensions
pub fn length_dimensions(v: &LengthOrPercentageOrAuto) -> Dimension {
    match v {
        LengthOrPercentageOrAuto::Length(l) => Dimension::Length(l.px),
        LengthOrPercentageOrAuto::Percentage(l) => Dimension::Percent(l.unit_value),
        LengthOrPercentageOrAuto::Auto => Dimension::Auto,
    }
}

/// layout style
pub fn node_layout_style(style: Arc<ComputedValues>, element: &ElementRef) -> Style {
    let physical_size = style.box_size().size_to_physical(style.writing_mode());
    let mut size = Size {
        width: length_dimensions(&physical_size.x),
        height: length_dimensions(&physical_size.y),
    };

    // get the img raw height/width
    if element.value().name() == "img" {
        let width = element.attr("width");
        let height = element.attr("height");
        if physical_size.x.inner_px() == 0.0 {
            match width {
                Some(w) => {
                    let w = w.parse::<f32>();
                    match w {
                        Ok(w) => {
                            size.width = length(w);
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
        if physical_size.y.inner_px() == 0.0 {
            match height {
                Some(h) => {
                    let h = h.parse::<f32>();

                    match h {
                        Ok(h) => {
                            size.height = length(h);
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
    }

    // todo: determine if all children at the top level have floats set to use flex-row
    Style {
        size,
        border: length(style.border_width().inner_px()),
        padding: length(style.padding().inner_px()),
        margin: length(style.margin().inner_px()),
        ..Default::default()
    }
}

/// push leaf
pub fn push_leaf<'a, 'b, 'c>(
    node: &NodeRef<'_, accessibility_scraper::Node>,
    author: &StyleSet,
    document: &'a Html,
    mut matching_context: &mut MatchingContext<'c, Simple>,
    taffy: &mut TaffyTree,
    l_leafs: &mut Vec<NodeId>,
) {
    match ElementRef::wrap(*node) {
        Some(element) => {
            let name = element.value().name();
            if !NODE_IGNORE.contains(name) {
                let style = accessibility_tree::style::cascade::style_for_element_ref(
                    &element,
                    &author,
                    &document,
                    &mut matching_context,
                );

                if node.has_children() {
                    let children = node.children();
                    let mut child_leafs: Vec<NodeId> = vec![];

                    // iterate all children and push into one leaf
                    for child in children {
                        push_leaf(
                            &child,
                            author,
                            document,
                            matching_context,
                            taffy,
                            &mut child_leafs,
                        );
                    }

                    l_leafs.push(
                        taffy
                            .new_with_children(node_layout_style(style, &element), &child_leafs)
                            .unwrap(),
                    );
                } else {
                    l_leafs.push(taffy.new_leaf(node_layout_style(style, &element)).unwrap());
                }
            }
        }
        _ => (),
    }
}

/// get a layout leaf a new leaf
pub fn leaf<'a, 'b, 'c>(
    element: &ElementRef,
    author: &StyleSet,
    document: &'a Html,
    mut matching_context: &mut MatchingContext<'c, Simple>,
    taffy: &mut TaffyTree,
) -> NodeId {
    let mut l_leafs: Vec<NodeId> = vec![];
    let mut children = element.children();

    while let Some(child) = children.next() {
        push_leaf(
            &child,
            author,
            document,
            matching_context,
            taffy,
            &mut l_leafs,
        );
    }

    let style = accessibility_tree::style::cascade::style_for_element_ref(
        &element,
        &author,
        &document,
        &mut matching_context,
    );

    let leaf_style = node_layout_style(style, &element);

    // build leaf with children
    if l_leafs.len() > 0 {
        taffy.new_with_children(leaf_style, &l_leafs)
    } else {
        taffy.new_leaf(leaf_style)
    }
    .unwrap()
}
