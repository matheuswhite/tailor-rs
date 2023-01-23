use std::fmt::{Display, Formatter, write};
use colored::*;

pub enum Command {
    Ok(String, String),
    InProgress(String, usize, usize),
    Fail(String),
    Warning(String),
    NoAction(String),
}

const FAIL_LABEL: &str = "Fail";
const WARNING_LABEL: &str = "Warning";
const ACTION_AREA: usize = 12;
const BAR_AREA: usize = 25;

impl Command {
    pub fn ok(action: &str, message: &str) -> Self {
        Command::Ok(action.to_string(), message.to_string())
    }

    pub fn in_progress(action: &str, current: usize, total: usize) -> Self {
        Command::InProgress(action.to_string(), current, total)
    }

    pub fn fail(message: &str) -> Self {
        Command::Fail(message.to_string())
    }

    pub fn warning(message: &str) -> Self {
        Command::Warning(message.to_string())
    }

    pub fn no_action(message: &str) -> Self {
        Command::NoAction(message.to_string())
    }

    fn spaces(text: &str) -> String {
        let amount = if text.chars().count() >= ACTION_AREA {
            0
        } else {
            ACTION_AREA - text.chars().count()
        };
        " ".repeat(amount)
    }

    fn bar(current: usize, total: usize) -> String {
        let bar_length = ((current as f32 / total as f32) * BAR_AREA as f32) as usize;
        let bar_remaining = BAR_AREA - bar_length;
        format!("[{}>{}]", "=".repeat(bar_length), " ".repeat(bar_remaining))
    }

    pub fn print(self) {
        println!("{}", self);
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::Ok(action, message) => {
                write!(f, "{}{} {}", Command::spaces(action), action.green().bold(), message)
            }
            Command::InProgress(action, current, total) => {
                write!(f, "{}{} {} {}/{}", Command::spaces(action), action.cyan().bold(), Command::bar(*current, *total), current, total)
            }
            Command::Fail(msg) => {
                write!(f, "{}{} {}", Command::spaces(FAIL_LABEL), FAIL_LABEL.red().bold(), msg)
            }
            Command::Warning(msg) => {
                write!(f, "{}{} {}", Command::spaces(WARNING_LABEL), WARNING_LABEL.yellow().bold(), msg)
            }
            Command::NoAction(msg) => {
                write!(f, "{} {}", " ".repeat(ACTION_AREA), msg)
            }
        }
    }
}
