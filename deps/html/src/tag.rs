use std::fmt;

#[derive(Debug)]
pub enum Content {
    Tag(Tag),
    Text(String),
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Content::Tag(tag) => tag.fmt(f),
            Content::Text(text) => write!(f, "{}", text),
        }
    }
}

#[derive(Debug)]
pub struct Tag {
    name: String,
    class_names: Vec<String>,
    contents: Vec<Content>,
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}>", self.name)?;
        for content in self.contents.iter() {
            content.fmt(f)?;
        }
        write!(f, "</{}>", self.name)
    }
}

impl Tag {
    pub fn new(name: String, class_names: Vec<String>, contents: Vec<Content>) -> Self {
        Self { name, class_names, contents }
    }
}
