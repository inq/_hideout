use html::{Content, Tag};
use std::fmt;

pub struct ContentWrapper<'a>(&'a Content);

impl<'a> ContentWrapper<'a> {
    pub fn new(content: &'a Content) -> Self {
        Self(content)
    }

    fn fmt_tag(tag: &Tag, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "html::Content::Tag(html::Tag::new(\"{}\".to_string(),",
            tag.name
        )?;
        write!(f, "vec![")?;
        for class_name in tag.class_names.iter() {
            write!(f, "\"{}\".to_string(),", class_name)?;
        }
        write!(f, "],")?;
        write!(f, "vec![")?;
        for content in tag.contents.iter() {
            write!(f, "{},", ContentWrapper(content))?;
        }
        write!(f, "]))")
    }
}

impl fmt::Display for ContentWrapper<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Content::Tag(tag) => Self::fmt_tag(tag, f),
            Content::Text(text) => write!(f, "html::Content::Text({}.to_string())", text),
        }
    }
}
