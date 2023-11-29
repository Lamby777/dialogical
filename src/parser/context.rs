use crate::comptime::{ScriptOutput, Unlink};
use crate::{Interaction, Link, LinkKVPair};

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

    pub fn drain_interactions(&mut self) -> Vec<Interaction> {
        let mut interactions = vec![];

        // TODO probably a better way to do this
        self.0.retain(|output| match output {
            ScriptOutput::Interaction(interaction) => {
                interactions.push(interaction.clone());
                false
            }
            _ => true,
        });

        interactions
    }

    pub fn link(&mut self, link: Link) {
        self.0.push(ScriptOutput::Link(link));
    }

    pub fn unlink(&mut self, unlink: &Unlink) {
        self.0.iter_mut().for_each(|output| {
            // TODO probably a bug in here
            if let ScriptOutput::Link(v) = output {
                v.associations
                    .retain(|other| !unlink.associations.contains(&other.0));
            }
        });

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
