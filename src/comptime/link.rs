use std::fmt;
use std::ops::Deref;

use super::ScriptError;

#[derive(Clone, Debug, PartialEq)]
pub struct LinkKVPair((String, String));

impl LinkKVPair {
    /// consume one line of split up words
    pub fn from_words<'a, I>(split: &mut I) -> Result<Self, ScriptError>
    where
        I: Iterator<Item = &'a str>,
    {
        let property = split.next().ok_or(ScriptError::InvalidLink)?.to_owned();
        Ok(LinkKVPair((property, split.collect::<Vec<_>>().join(" "))))
    }

    pub fn from_tuple(pair: (&str, &str)) -> Self {
        Self::from_slices(pair.0, pair.1)
    }

    pub fn from_slices(property: &str, target: &str) -> Self {
        Self((property.to_owned(), target.to_owned()))
    }
}

impl Deref for LinkKVPair {
    type Target = (String, String);
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// One section of link commands...
#[derive(Clone, Debug, PartialEq)]
pub struct Link {
    pub from: LinkKVPair,
    pub linked: Vec<LinkKVPair>,
    pub negative: bool,
}

/// Target = thing to track
/// Association = thing to link to the target
///
/// In practice:
/// Link NAME <Target>
/// VOX <Association>
impl Link {
    pub fn new(property: &str, target: &str) -> Self {
        let pair = LinkKVPair::from_slices(property, target);
        Self::from_pair(pair)
    }

    pub fn from_pair(from: LinkKVPair) -> Self {
        Self {
            from,
            linked: vec![],
            negative: false,
        }
    }

    pub fn add_association(&mut self, pair: LinkKVPair) {
        self.linked.push(pair);
    }
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?}", self.from)?;

        for link in &self.linked {
            writeln!(f, " -> {:?}", link)?;
        }

        Ok(())
    }
}
