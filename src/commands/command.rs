pub trait Command {
    fn parse_args(&mut self, args: &[String]) -> Option<()>;

    fn execute(&self) -> Result<(), String>;
}
