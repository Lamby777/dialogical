pub struct AutolinkKVPair(String, String);

pub struct Autolink {
    from: AutolinkKVPair,
    linked: Vec<AutolinkKVPair>,
}

impl Autolink {
    pub fn new(property: &str, target: &str) -> Self {
        let pair = AutolinkKVPair(property.to_owned(), target.to_owned());
        Self::from_pair(pair)
    }

    pub fn from_pair(from: AutolinkKVPair) -> Self {
        Self {
            from,
            linked: vec![],
        }
    }
}
