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

#[derive(Default)]
pub struct PageMetadata {
    pub speaker: Metadata<String>,
    pub vox: Metadata<String>,
}

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
