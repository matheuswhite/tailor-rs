pub mod build_pkg;
pub mod clean_pkg;
pub mod new_pkg;
pub mod run_pkg;

pub trait Command {
    fn parse_args(&mut self, args: &[String]) -> Result<bool, String>;

    fn execute(&self) -> Result<(), String>;
}
