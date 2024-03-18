use crate::engine::styles::layout::leaf;
use accessibility_scraper::selector::Simple;
use accessibility_scraper::ElementRef;
use accessibility_scraper::Html;
use accessibility_tree::style::StyleSet;
use selectors::matching::MatchingContext;
use std::collections::BTreeMap;
use std::collections::HashSet;
use taffy::prelude::*;

lazy_static! {
    static ref NODE_IGNORE: HashSet<&'static str> =
        HashSet::from(["meta", "style", "link", "script", "head", "html", "body"]);
}

/// try to fix all possible issues using a spec against the tree.
pub fn parse_accessibility_tree<'a, 'b, 'c>(
    document: &'a Html,
    _author: &StyleSet,
    match_context: MatchingContext<'c, Simple>, // todo: return the nodes with a tuple of the layout node and the element node
) -> (
    BTreeMap<&'a str, Vec<(ElementRef<'a>, Option<NodeId>)>>,
    Option<TaffyTree>,
    MatchingContext<'c, Simple>,
) {
    let mut accessibility_tree: BTreeMap<&str, Vec<(ElementRef<'_>, Option<NodeId>)>> =
        BTreeMap::from(if document.root_element().value().name() == "html" {
            [("title".into(), Default::default())]
        } else {
            [(Default::default(), Default::default())]
        });
    for node in document.tree.nodes() {
        match ElementRef::wrap(node) {
            Some(element) => {
                let name = element.value().name();
                accessibility_tree
                    .entry(name)
                    .and_modify(|n| n.push((element, None)))
                    .or_insert(Vec::from([(element, None)]));
            }
            _ => (),
        };
    }

    (accessibility_tree, None, match_context)
}

/// try to fix all possible issues using a spec against the tree with bounding boxs.
pub fn parse_accessibility_tree_bounded<'a, 'b, 'c>(
    document: &'a Html,
    author: &StyleSet,
    match_context: MatchingContext<'c, Simple>, // todo: return the nodes with a tuple of the layout node and the element node
) -> (
    BTreeMap<&'a str, Vec<(ElementRef<'a>, Option<NodeId>)>>,
    Option<TaffyTree>,
    MatchingContext<'c, Simple>,
) {
    let mut taffy = TaffyTree::new();
    let mut accessibility_tree: BTreeMap<&str, Vec<(ElementRef<'_>, Option<NodeId>)>> =
        BTreeMap::from(if document.root_element().value().name() == "html" {
            [("title".into(), Default::default())]
        } else {
            [(Default::default(), Default::default())]
        });
    let mut matching_context = match_context;
    let mut layout_leafs: Vec<NodeId> = vec![];

    // push taffy layout in order from elements
    for node in document.tree.nodes() {
        match ElementRef::wrap(node) {
            Some(element) => {
                let name = element.value().name();
                let layout_leaf = {
                    if NODE_IGNORE.contains(name) {
                        taffy.new_leaf(Default::default()).unwrap()
                    } else {
                        leaf(
                            &element,
                            &author,
                            document,
                            &mut matching_context,
                            &mut taffy,
                        )
                    }
                };
                accessibility_tree
                    .entry(name)
                    .and_modify(|n| n.push((element, Some(layout_leaf))))
                    .or_insert(Vec::from([(element, Some(layout_leaf))]));
            }
            _ => (),
        };
    }

    match accessibility_tree.get("body") {
        Some(node) => {
            for child in node[0].0.children() {
                match ElementRef::wrap(child) {
                    Some(element) => {
                        if !NODE_IGNORE.contains(element.value().name()) {
                            let leaf = leaf(
                                &element,
                                &author,
                                document,
                                &mut matching_context,
                                &mut taffy,
                            );

                            layout_leafs.push(leaf)
                        }
                    }
                    _ => (),
                }
            }
        }
        _ => (),
    };

    let root_node = taffy
        .new_with_children(
            Style {
                flex_direction: FlexDirection::Column,
                // compute the default layout from CDP
                size: Size {
                    width: length(800.0),
                    height: length(600.0),
                },
                ..Default::default()
            },
            &layout_leafs,
        )
        .unwrap();

    // println!("Layout leafs {:?}", layout_leafs.len());

    // TODO: set the root node to html, body in the accessibility_tree
    taffy.compute_layout(root_node, Size::MAX_CONTENT).unwrap();

    // for lea in layout_leafs {
    //     println!("Leaf Position {:?}", taffy.layout(lea).unwrap());
    // }

    (accessibility_tree, Some(taffy), matching_context)
}
