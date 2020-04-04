mod debug;

use crate::http::Method;
use crate::util::AssetStore;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub enum Handler {
    Resource(usize),
    // TODO: Pass request object
    Arg0(fn(&[u8]) -> crate::http::Response),
    Arg1(fn(&[u8], &str) -> crate::http::Response),
    Arg2(fn(&[u8], &str, &str) -> crate::http::Response),
}

#[derive(Debug)]
pub enum Args<'a> {
    Arg0,
    Arg1(&'a str),
    Arg2(&'a str, &'a str),
    Arg3(&'a str, &'a str, &'a str),
}

impl<'a> Args<'a> {
    pub fn push(&mut self, data: &'a str) {
        *self = match self {
            Args::Arg0 => Args::Arg1(data),
            Args::Arg1(a1) => Args::Arg2(a1, data),
            Args::Arg2(a1, a2) => Args::Arg3(a1, a2, data),
            _ => unimplemented!(),
        };
    }
}

#[derive(Default)]
pub struct Router {
    inner: HashMap<crate::http::Method, Node>,
    pub asset_store: AssetStore,
}

#[derive(Default)]
pub struct Node {
    children: HashMap<String, Node>,
    variable: Option<Box<Node>>,
    handler: Option<Handler>,
    url: Option<String>,
}

impl Router {
    pub fn new(asset_store: AssetStore) -> Self {
        Self {
            inner: Default::default(),
            asset_store,
        }
    }

    pub fn add_get(&mut self, path: &str, handler: Handler) -> &mut Self {
        self.add_path(Method::Get, path, handler)
    }

    pub fn add_post(&mut self, path: &str, handler: Handler) -> &mut Self {
        self.add_path(Method::Post, path, handler)
    }

    pub fn add_path(&mut self, method: Method, path: &str, handler: Handler) -> &mut Self {
        let mut current_node = self.inner.entry(method).or_insert_with(Node::default);
        let mut elems = path.split('/');

        // Path should start with '/'
        assert!(elems.next().unwrap().is_empty());
        if path.len() > 1 {
            for elem in elems {
                if elem.starts_with(':') {
                    // Variable
                    if current_node.variable.is_some() {
                        current_node = current_node.variable.as_mut().unwrap();
                    } else {
                        current_node.variable = Some(Default::default());
                        current_node = current_node.variable.as_mut().unwrap();
                    }
                } else {
                    // Constant
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
        }
        current_node.url = Some(path.to_string());
        current_node.handler = Some(handler);
        self
    }

    pub fn route<'a>(&self, method: Method, uri: &'a str) -> Option<(Handler, Args<'a>)> {
        let mut current_node = self.inner.get(&method)?;
        let mut from = 0;
        let mut args = Args::Arg0; // TODO: Implement

        for (i, c) in uri.char_indices() {
            if c == '/' || c == '?' {
                if from > 0 {
                    let elem = &uri[from..i];
                    if current_node.children.contains_key(elem) {
                        current_node = current_node.children.get(elem).unwrap();
                    } else if current_node.variable.is_some() {
                        args.push(elem);
                        current_node = current_node.variable.as_ref().unwrap();
                    } else {
                        log::warn!("Route error: {}({})", uri, elem);
                        return None;
                    }
                }

                if c == '?' {
                    // TODO: Process query string
                    return current_node.handler.map(|handler| (handler, args));
                }
                from = i + 1;
            }
        }

        if from == uri.len() {
            // Ends with '/'
            current_node.handler
        } else {
            let elem = &uri[from..];
            if current_node.children.contains_key(elem) {
                current_node.children.get(elem).unwrap()
            } else if current_node.variable.is_some() {
                args.push(elem);
                current_node.variable.as_ref().unwrap()
            } else {
                log::warn!("Route error: {}({})", uri, elem);
                return None;
            }
            .handler
        }
        .map(|handler| (handler, args))
    }

    pub fn to_debug(&self) -> debug::Router {
        debug::Router {
            inner: self
                .inner
                .iter()
                .map(|(method, root)| (*method, root.to_debug("/", 0)))
                .collect(),
        }
    }
}

impl Node {
    fn to_debug(&self, label: &str, indent: usize) -> debug::Node {
        let mut children: Vec<debug::Node> = self
            .children
            .iter()
            .map(|(key, node)| node.to_debug(key, indent + 1))
            .collect();
        if let Some(variable) = self.variable.as_ref() {
            children.push(variable.to_debug(":VAR", indent + 1));
        };

        debug::Node {
            indent,
            label: label.to_string(),
            children,
            is_leaf: self.url.is_some(),
        }
    }
}
