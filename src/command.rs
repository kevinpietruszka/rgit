use crate::result::RgitResult;

pub trait RgitCommand {
    fn run(&self) -> RgitResult;
}
