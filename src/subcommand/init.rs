use crate::constants::{RGIT_DIR, RgitCommand, RgitResult};
use std::fs;

pub struct InitCommand {}

impl InitCommand {
    pub fn new() -> Self {
        InitCommand {}
    }
}

impl RgitCommand for InitCommand {
    fn run(&self) -> Vec<RgitResult> {
        let mut output = vec![];

        match fs::create_dir(RGIT_DIR) {
            Ok(()) => output.push(RgitResult::Success(
                "Successfully created a rgit repository".to_string(),
            )),
            Err(e) => output.push(RgitResult::Fatal(
                "Failure to create the repository".to_string(),
            )),
        }

        return output;
    }
}
