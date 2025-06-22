use crate::command::RgitCommand;
use crate::constants::{RGIT_DIR, SUB_DIRS};
use crate::result::RgitResult;
use std::fs;
use std::path;

pub struct InitCommand {}

impl InitCommand {
    pub fn new() -> Self {
        InitCommand {}
    }
}

impl RgitCommand for InitCommand {
    fn run(&self) -> RgitResult {
        if path::Path::new(RGIT_DIR).exists() {
            return RgitResult::Warning(format!(
                "Repository already exists in {} directory",
                RGIT_DIR
            ));
        }
        match fs::create_dir(RGIT_DIR) {
            Ok(()) => (),
            Err(e) => {
                return RgitResult::Fatal(format!(
                    "Could not create the repository with error ({})",
                    e
                ));
            }
        }

        let mut errors = vec![];

        for dir in SUB_DIRS {
            match fs::create_dir(dir) {
                Ok(()) => (),
                Err(e) => errors.push(format!(
                    "\nFailed to create {} subdir with error {}",
                    dir, e
                )),
            }
        }

        if errors.len() > 0 {
            return RgitResult::Warning(format!(
                "Repository created with warnings:{}",
                errors.join("")
            ));
        }

        RgitResult::Success(format!("Created repository in {} directory", RGIT_DIR))
    }
}
