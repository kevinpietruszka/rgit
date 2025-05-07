use std::fmt;

pub const RGIT_DIR: &str = ".rgit";

pub enum RgitResult {
    Success(String),
    Warning(String),
    Fatal(String),
}

impl fmt::Display for RgitResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RgitResult::Success(msg) => write!(f, "Success: {}", msg),
            RgitResult::Warning(msg) => write!(f, "Warning: {}", msg),
            RgitResult::Fatal(msg) => write!(f, "Fatal: {}", msg),
        }
    }
}

pub trait RgitCommand {
    fn run(&self) -> Vec<RgitResult>;
}
