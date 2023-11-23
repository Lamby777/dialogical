use super::ScriptError;

#[derive(Clone, Debug)]
pub struct LinkKVPair(String, String);

impl LinkKVPair {
    /// consume one line of split up words
    pub fn from_words<'a, I>(split: &mut I) -> Result<Self, ScriptError>
    where
        I: Iterator<Item = &'a str>,
    {
        let property = split.next().ok_or(ScriptError::InvalidLink)?.to_owned();
        Ok(LinkKVPair(property, split.collect::<Vec<_>>().join(" ")))
    }
}

/// One section of link commands...
#[derive(Clone, Debug)]
pub struct Link {
    from: LinkKVPair,
    linked: Vec<LinkKVPair>,
}

impl Link {
    pub fn new(property: &str, target: &str) -> Self {
        let pair = LinkKVPair(property.to_owned(), target.to_owned());
        Self::from_pair(pair)
    }

    pub fn from_pair(from: LinkKVPair) -> Self {
        Self {
            from,
            linked: vec![],
        }
    }

    pub fn add_link(&mut self, pair: LinkKVPair) {
        self.linked.push(pair);
    }
}
