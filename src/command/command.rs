use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct Command {
    pub command: String,
    pub arguments: Vec<String>,
}

impl Command {
    pub fn parse(text: String) -> Option<Self> {
        let text = text.trim();
        if text.is_empty() {
            return None;
        }

        let mut chunks: Vec<&str> = text.split(' ').collect();
        return if let Some(command) = chunks.pop() {
            Some(
                Self {
                    command: String::from(command),
                    arguments: chunks.iter().map(|&arg| arg.into()).collect(),
                }
            )
        } else {
            None
        }
    }
}