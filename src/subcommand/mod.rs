pub mod init;

pub trait Runnable {
    fn run(&self) -> std::io::Result<String>;
}
