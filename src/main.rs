mod dom;

fn main() {
    let dom_tree = dom::elem(
        "body".to_string(),
        dom::AttrMap::new(),
        vec![
            dom::elem(
                "h1".to_string(),
                dom::AttrMap::new(),
                vec![dom::text("hello kakarot".to_string())],
            ),
            dom::text("hello".to_string()),
        ],
    );
    dom::walk_dom_tree(&dom_tree, 0);
}
