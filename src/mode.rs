#[derive(Default)]
pub enum Mode {
    #[default]
    Debug,
    Release,
}

impl TryFrom<&str> for Mode {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "--debug" => Ok(Mode::Debug),
            "--release" => Ok(Mode::Release),
            _ => Err(format!("invalid mode: {}", value)),
        }
    }
}

impl ToString for Mode {
    fn to_string(&self) -> String {
        match self {
            Mode::Debug => "debug".to_string(),
            Mode::Release => "release".to_string(),
        }
    }
}
