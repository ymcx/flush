use std::collections::HashSet;

pub struct Flags {
    pub all: bool,
    pub full_path: bool,
    pub help: bool,
    pub long: bool,
}

impl Flags {
    pub fn new(flags: &HashSet<String>) -> Self {
        Self {
            all: flags.contains("a"),
            full_path: flags.contains("f"),
            help: flags.contains("h"),
            long: flags.contains("l"),
        }
    }
}
