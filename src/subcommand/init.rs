use crate::command::RgitCommand;
use crate::constants::{RGIT_DIR};
use crate::result::RgitResult;
use std::fs;

pub struct InitCommand {}

impl InitCommand {
    pub fn new() -> Self {
        InitCommand {}
    }
}

impl RgitCommand for InitCommand {
    fn run(&self) -> RgitResult {
        match fs::create_dir(RGIT_DIR) {
            Ok(()) => RgitResult::Success(format!("Created repository in {} directory", RGIT_DIR)),
            Err(e) => RgitResult::Fatal(format!("Could not create the repository with error ({})", e)),
        }
    }
}
