use maplit::hashmap;
use std::collections::HashMap;

macro_rules! impl_with_attrs {
    ($t:ty) => {
        pub fn with_attrs(mut self, attrs: &[(&'a str, &'a str)]) -> Self {
            attrs.iter().for_each(|(k, v)| {
                self.attrs.entry(k.to_string()).or_insert(v.to_string());
            });
            self
        }

        pub fn get_attr(&self, attr: &str) -> Option<&str> {
            let to_find = attr.to_string();
            if let Some(i) = self.attrs.get(&to_find) {
                Some(i.as_str())
            } else {
                None
            }
        }
    };
}

pub mod graph {
    use super::*;

    pub mod graph_items {
        use super::*;

        pub mod edge {
            use super::*;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Edge<'a> {
                pub ends: (&'a str, &'a str),
                pub attrs: HashMap<String, String>,
            }

            impl<'a> Edge<'a> {
                pub fn new(a: &'a str, b: &'a str) -> Self {
                    Edge {
                        ends: (a, b),
                        attrs: hashmap! {},
                    }
                }

                impl_with_attrs!(Edge);
            }
        }

        pub mod node {
            use super::*;

            #[derive(Debug, Clone, PartialEq)]
            pub struct Node<'a> {
                pub name: &'a str,
                pub attrs: HashMap<String, String>,
            }
            impl<'a> Node<'a> {
                pub fn new(n: &'a str) -> Self {
                    Node {
                        name: n,
                        attrs: hashmap! {},
                    }
                }
                impl_with_attrs!(Node);
            }
        }
    }

    pub use graph_items::edge::Edge;
    pub use graph_items::node::Node;

    pub struct Graph<'a> {
        pub edges: Vec<Edge<'a>>,
        pub nodes: Vec<Node<'a>>,
        pub attrs: HashMap<String, String>,
    }

    impl<'a> Graph<'a> {
        pub fn new() -> Self {
            Graph {
                edges: Vec::new(),
                nodes: Vec::new(),
                attrs: hashmap! {},
            }
        }

        pub fn with_nodes(mut self, nodes: &[Node<'a>]) -> Graph<'a> {
            self.nodes.extend_from_slice(nodes);
            self
        }

        pub fn with_edges(mut self, edges: &[Edge<'a>]) -> Graph<'a> {
            self.edges.extend_from_slice(edges);
            self
        }

        pub fn get_node(&self, target: &str) -> Option<Node> {
            self.nodes.iter().cloned().find(|n| n.name == target)
        }

        impl_with_attrs!(Graph);
    }
}
