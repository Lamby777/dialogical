use crate::comptime::{ScriptOutput, Unlink};
use crate::{InteractionMap, Link, LinkKVPair};

/// Wrapper type around `Vec<ScriptOutput>`.
///
/// Any result of comptime execution that affects
/// how the parser does its job goes in here...
#[derive(Debug, Default, PartialEq)]
pub struct ScriptContext(pub Vec<ScriptOutput>);

#[allow(unused)] // STFU!!!!
impl ScriptContext {
    pub fn log(&mut self, msg: &str) {
        self.0.push(ScriptOutput::LogMessage(msg.to_owned()));
    }

    pub fn drain_interactions(&mut self) -> InteractionMap {
        let mut interactions = InteractionMap::new();

        // TODO probably a better way to do this
        self.0.retain(|output| match output {
            ScriptOutput::Interaction(ix_id, ix) => {
                interactions.insert(ix_id.to_owned(), ix.clone());
                false
            }
            _ => true,
        });

        interactions
    }

    pub fn link(&mut self, link: Link) {
        // TODO consider it an error to have an empty (un)link?
        if link.associations.is_empty() {
            return;
        }

        self.0.push(ScriptOutput::Link(link));
    }

    pub fn unlink(&mut self, unlink: &Unlink) {
        if unlink.associations.is_empty() {
            return;
        }

        for link in self.iter_links_mut() {
            if link.target == unlink.target {
                link.associations.retain(|v| v.0 != unlink.associations[0]);
            }
        }

        self.clean_links();
    }

    /// Delete any links with empty associations
    pub fn clean_links(&mut self) {
        self.0.retain(|output| match output {
            ScriptOutput::Link(link) => !link.associations.is_empty(),
            _ => true,
        });
    }

    pub fn fmt_links(&self) -> String {
        self.iter_links()
            .map(|v| format!("{}", v))
            .collect::<String>()
    }

    /// Iterator over all the log messages
    pub fn iter_logs(&self) -> impl Iterator<Item = &str> {
        self.0.iter().filter_map(|v| match v {
            ScriptOutput::LogMessage(msg) => Some(msg.as_str()),
            _ => None,
        })
    }

    pub fn logs(&self) -> Vec<&str> {
        self.iter_logs().collect()
    }

    pub fn iter_links(&self) -> impl Iterator<Item = &Link> {
        self.0.iter().filter_map(|v| match v {
            ScriptOutput::Link(link) => Some(link),
            _ => None,
        })
    }

    pub fn iter_links_mut(&mut self) -> impl Iterator<Item = &mut Link> {
        self.0.iter_mut().filter_map(|v| match v {
            ScriptOutput::Link(link) => Some(link),
            _ => None,
        })
    }

    pub fn links(&self) -> Vec<&Link> {
        self.iter_links().collect()
    }

    pub fn find_links_for(&mut self, kv: LinkKVPair) -> Vec<LinkKVPair> {
        self.links()
            .iter()
            .filter_map(|v| {
                if v.target == kv {
                    Some(v.associations.clone())
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    }
}
