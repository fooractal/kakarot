use std::collections::HashMap;

pub struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}

pub enum NodeType {
    Text(String),
    Element(ElementData),
}

pub struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

pub type AttrMap = HashMap<String, String>;

pub fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    }
}

pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        }),
    }
}

pub fn walk_dom_tree(root: &Node, indent: i32) {
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
