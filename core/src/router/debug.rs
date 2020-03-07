use crate::http::Method;
use std::collections::HashMap;
use std::fmt::{self, Formatter};

pub struct Router {
    pub(super) inner: HashMap<Method, Node>,
}

impl fmt::Display for Router {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Router {{")?;
        for (method, node) in self.inner.iter() {
            write!(f, "{:?}\n{:?}", method, node)?;
        }
        write!(f, "}}")
    }
}
pub struct Node {
    pub(super) indent: usize,
    pub(super) label: String,
    pub(super) children: Vec<Node>,
    pub(super) is_leaf: bool,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let indent = (0..self.indent).map(|_| "  ").collect::<String>();
        writeln!(
            f,
            "{}{}{}",
            if self.is_leaf { " * " } else { "   " },
            indent,
            self.label
        )?;
        for child in self.children.iter() {
            write!(f, "{:?}", child)?;
        }
        Ok(())
    }
}
