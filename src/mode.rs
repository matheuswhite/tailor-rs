#[derive(Clone, Copy, Default)]
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

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Debug => write!(f, "debug"),
            Mode::Release => write!(f, "release"),
        }
    }
}
