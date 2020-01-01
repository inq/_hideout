use proc_macro::{token_stream, TokenTree};

#[derive(Debug)]
pub enum Node {
    Ident(String),
    Punct(char),
    Literal(String),
    NotImplemented,
}

#[derive(Default)]
pub struct LineBuilder {
    level: Option<usize>,
    column: Option<usize>,
    nodes: Vec<Node>,
}

impl LineBuilder {
    // TODO: Replace Option with Result
    fn build(self) -> Option<Line> {
        Some(Line {
            level: self.level?,
            nodes: self.nodes,
        })
    }

    fn put(&mut self, token: TokenTree) {
        let span = token.span();
        if self.level.is_none() {
            self.level = Some(span.start().column);
        }

        let node = match token {
            TokenTree::Ident(ident) => Node::Ident(ident.to_string()),
            TokenTree::Punct(punct) => Node::Punct(punct.as_char()),
            TokenTree::Literal(literal) => Node::Literal(literal.to_string()),
            TokenTree::Group(_) => Node::NotImplemented,
        };
        self.nodes.push(node);
        self.column = Some(span.end().column);
    }
}

#[derive(Debug)]
pub struct Line {
    level: usize,
    nodes: Vec<Node>,
}

#[derive(Debug)]
pub struct Parser {
    lines: Vec<Line>,
}

impl Parser {
    pub fn from_tokens(tokens: token_stream::IntoIter) -> Option<Self> {
        let mut current_line_num = None;
        let mut lines = vec![];
        let mut current_line = LineBuilder::default();

        for token in tokens {
            let span = token.span();
            let start = span.start();
            if current_line_num.map_or(false, |line_num| line_num != start.line) {
                lines.push(std::mem::take(&mut current_line).build()?);
            }
            current_line_num = Some(start.line);
            current_line.put(token);
        }
        lines.push(current_line.build()?);
        Some(Self { lines })
    }
}
