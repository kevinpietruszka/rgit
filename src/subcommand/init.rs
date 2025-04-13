use crate::constants::RGIT_DIR;
use std::fs;

use super::Runnable;

pub struct InitCommand {}

impl InitCommand {
    pub fn new() -> Self {
        InitCommand {}
    }
}

impl Runnable for InitCommand {
    fn run(&self) -> Result<(), std::io::Error> {
        fs::create_dir(RGIT_DIR)?;
        Ok(())
    }
}
