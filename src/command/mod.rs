pub mod build_pkg;
pub mod new_pkg;
pub mod run_pkg;

pub trait Command {
    fn parse_args(&mut self, args: &[String]) -> Option<()>;

    fn execute(&self) -> Result<(), String>;
}
