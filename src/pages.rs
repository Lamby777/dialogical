//!
//! Data structures used by the parser
//!
//! TODO less `String`, more `&'a str`
//!

#[derive(Debug, PartialEq)]
pub struct Interaction<'a> {
    pub id: &'a str,
    pub pages: Vec<Page>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Page {
    pub metadata: PageMetadata,
    pub content: String,
}

impl Page {
    pub fn from_content(content: String) -> Self {
        Self {
            content,
            metadata: PageMetadata::default(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PageMetadata {
    pub speaker: Metadata<String>,
    pub vox: Metadata<String>,
}

impl PageMetadata {
    /// shorthand for permanent change of speaker and vox with same string
    /// good for writing quick unit tests
    pub fn new_perm_double(speaker: &str) -> Self {
        let meta = Metadata::Permanent(speaker.to_owned());

        Self {
            speaker: meta.clone(),
            vox: meta,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Metadata<T> {
    Permanent(T),
    PageOnly(T),
    NoChange,
}

impl<T> Default for Metadata<T> {
    fn default() -> Self {
        Self::NoChange
    }
}
