use std::collections::HashMap;

struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}

enum NodeType {
    Text(String),
    Element(ElementData),
}

struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

type AttrMap = HashMap<String, String>;

fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    }
}

fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        }),
    }
}

fn walk_dom_tree(root: &Node, indent: i32) {
    for _ in 0..indent {
        print!(" ");
    }

    match &root.node_type {
        NodeType::Text(content) => println!("\"{}\"", content),
        NodeType::Element(data) => {
            println!("<{}> {:?}", data.tag_name, data.attributes);

            for child in &root.children {
                walk_dom_tree(child, indent + 2);
            }
        }
    }
}

fn main() {
    let dom_tree = elem(
        "body".to_string(),
        HashMap::new(),
        vec![
            elem(
                "h1".to_string(),
                HashMap::new(),
                vec![text("hello kakarot".to_string())],
            ),
            text("hello".to_string()),
        ],
    );
    walk_dom_tree(&dom_tree, 0);
}
