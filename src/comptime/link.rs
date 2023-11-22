pub struct LinkKVPair(String, String);

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
}
