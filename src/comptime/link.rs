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

/// One section of (un)link commands...
#[derive(Clone, Debug, PartialEq)]
pub struct LinkLike<A> {
    pub target: LinkKVPair,
    pub associations: Vec<A>,
}

pub type Link = LinkLike<LinkKVPair>;
pub type Unlink = LinkLike<String>;

/// Target = thing to track
/// Association = thing to link to the target
///
/// In practice:
/// ```
/// Link <Target Key> <Target Value>
/// <Association Key> <Association Value>
/// ```
impl<A> LinkLike<A> {
    pub fn new(target_key: &str, target_value: &str) -> Self {
        let pair = LinkKVPair::from_slices(target_key, target_value);
        Self::from_pair(pair)
    }

    pub fn from_pair(target: LinkKVPair) -> Self {
        Self {
            target,
            associations: vec![],
        }
    }

    pub fn add_association(&mut self, pair: A) {
        self.associations.push(pair);
    }
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?}", self.target)?;

        for link in &self.associations {
            writeln!(f, " -> {:?}", link)?;
        }

        Ok(())
    }
}
