use crate::comptime::ScriptOutput;
use crate::{Link, LinkKVPair};

/// Wrapper type around `Vec<ScriptOutput>`.
///
/// Any result of comptime execution that affects
/// how the parser does its job goes in here...
#[derive(Debug, Default, PartialEq)]
pub struct ScriptContext(pub Vec<ScriptOutput>);

impl ScriptContext {
    pub fn log(&mut self, msg: &str) {
        self.0.push(ScriptOutput::LogMessage(msg.to_owned()));
    }

    pub fn link(&mut self, link: Link) {
        self.0.push(ScriptOutput::Link(link));
    }

    pub fn unlink(&mut self, link: &Link) {
        println!(
            "Pre Unlink:\n{}",
            self.iter_links()
                .map(|v| format!("{}", v))
                .collect::<String>()
        );

        self.0.iter_mut().for_each(|output| {
            if let ScriptOutput::Link(v) = output {
                v.linked.retain(|other| !link.linked.contains(other));
            }
        });

        println!(
            "Pre Unlink:\n{}",
            self.iter_links()
                .map(|v| format!("{}", v))
                .collect::<String>()
        );
        println!("\n\n\n\n\n");
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

    pub fn all_links(&self) -> Vec<&Link> {
        self.iter_links().collect()
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
