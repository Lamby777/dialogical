use super::ScriptError;

#[derive(Clone, Debug, PartialEq)]
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

    pub fn from_tuple(pair: (&str, &str)) -> Self {
        Self(pair.0.to_owned(), pair.1.to_owned())
    }
}

impl From<LinkKVPair> for (String, String) {
    fn from(pair: LinkKVPair) -> Self {
        (pair.0, pair.1)
    }
}

impl From<(String, String)> for LinkKVPair {
    fn from(pair: (String, String)) -> Self {
        Self(pair.0, pair.1)
    }
}

/// One section of link commands...
#[derive(Clone, Debug)]
pub struct Link {
    pub from: LinkKVPair,
    pub linked: Vec<LinkKVPair>,
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
