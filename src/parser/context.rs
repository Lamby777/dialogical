use crate::comptime::ScriptOutput;
use crate::{Link, LinkKVPair};

/// Wrapper type around `Vec<ScriptOutput>`.
///
/// Any result of comptime execution that affects
/// how the parser does its job goes in here...
#[derive(Default)]
pub struct ScriptContext(Vec<ScriptOutput>);

impl ScriptContext {
    pub fn log(&mut self, msg: &str) {
        self.0.push(ScriptOutput::LogMessage(msg.to_owned()));
    }

    pub fn link(&mut self, link: Link) {
        self.0.push(ScriptOutput::Link(link.clone()));
    }

    pub fn unlink(&mut self, link: &Link) {
        let _ = self.0.iter_mut().map(|output| {
            if let ScriptOutput::Link(v) = output {
                v.linked.retain(|other| !link.linked.contains(other));
            }
        });
    }

    pub fn all_links(&self) -> Vec<&Link> {
        self.0
            .iter()
            .filter_map(|v| match v {
                ScriptOutput::Link(link) => Some(link),
                _ => None,
            })
            .collect()
    }

    pub fn get_links_for(&mut self, kv: LinkKVPair) -> Vec<LinkKVPair> {
        self.all_links()
            .iter()
            .filter_map(|v| {
                if v.from == kv {
                    Some(v.linked.clone())
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    }
}
