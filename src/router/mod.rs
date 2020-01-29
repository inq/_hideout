use std::collections::HashMap;
pub type Handler = fn(&str) -> crate::http::Response;

pub struct Router {
    root: Node,
}

#[derive(Default)]
pub struct Node {
    children: HashMap<String, Node>,
    handler: Option<Handler>,
    url: Option<String>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            root: Node::default(),
        }
    }

    pub fn add_path(&mut self, path: &str, handler: Handler) -> &mut Self {
        let mut current_node = &mut self.root;
        let mut elems = path.split('/');

        // Path should start with '/'
        assert!(elems.next().unwrap().is_empty());
        if path.len() > 1 {
            for elem in elems {
                if current_node.children.contains_key(elem) {
                    current_node = current_node.children.get_mut(elem).unwrap();
                } else {
                    current_node
                        .children
                        .insert(elem.to_string(), Node::default());
                    current_node = current_node.children.get_mut(elem).unwrap();
                }
            }
        }
        current_node.url = Some(path.to_string());
        current_node.handler = Some(handler);
        self
    }

    pub fn route(&self, uri: &str) -> Option<Handler> {
        let mut current_node = &self.root;
        let mut from = 0;

        for (i, c) in uri.char_indices() {
            if c == '/' || c == '?' {
                if from > 0 {
                    let elem = &uri[from..i];
                    if current_node.children.contains_key(elem) {
                        current_node = current_node.children.get(elem).unwrap();
                    } else {
                        log::warn!("Route error: {}({})", uri, elem);
                        return None;
                    }
                }

                if c == '?' {
                    // TODO: Process query string
                    return current_node.handler;
                }
                from = i + 1;
            }
        }

        if from == uri.len() {
            // Ends with '/'
            current_node.handler
        } else {
            current_node
                .children
                .get(&uri[from..])
                .and_then(|child| child.handler)
        }
    }
}
