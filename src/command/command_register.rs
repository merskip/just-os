use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use crate::command::command::Command;
use crate::println;

pub struct CommandRegister {
    commands: BTreeMap<String, Box<dyn Fn(Command)>>
}

impl CommandRegister {
    pub fn new() -> Self {
        Self {
            commands: BTreeMap::new(),
        }
    }

    pub fn register(&mut self, command: &str, handler: Box<dyn Fn(Command)>) {
        self.commands.insert(command.into(), handler);
    }

    pub fn perform(&self, command: Command) {
        if let Some(handler) = self.commands.get(&*command.command) {
            handler(command);
        } else {
            println!("Not found command: {}", command.command);
        }
    }
}
