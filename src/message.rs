use std::fmt::{Display, Formatter};
use colored::*;

pub enum Message {
    Ok(String, String),
    InProgress(String, usize, usize),
    Fail(String),
    Warning(String),
    #[allow(dead_code)]
    NoAction(String),
}

const FAIL_LABEL: &str = "Fail";
const WARNING_LABEL: &str = "Warning";
const ACTION_AREA: usize = 12;
const BAR_AREA: usize = 25;

impl Message {
    pub fn ok(title: &str, description: &str) -> Self {
        Message::Ok(title.to_string(), description.to_string())
    }

    pub fn in_progress(title: &str, current: usize, total: usize) -> Self {
        Message::InProgress(title.to_string(), current, total)
    }

    pub fn fail(description: &str) -> Self {
        Message::Fail(description.to_string())
    }

    pub fn warning(description: &str) -> Self {
        Message::Warning(description.to_string())
    }

    #[allow(dead_code)]
    pub fn no_action(message: &str) -> Self {
        Message::NoAction(message.to_string())
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

impl Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::Ok(title, description) => {
                write!(f, "{}{} {}", Message::spaces(title), title.green().bold(), description)
            }
            Message::InProgress(title, current, total) => {
                write!(f, "{}{} {} {}/{}", Message::spaces(title), title.cyan().bold(), Message::bar(*current, *total), current, total)
            }
            Message::Fail(description) => {
                write!(f, "{}{} {}", Message::spaces(FAIL_LABEL), FAIL_LABEL.red().bold(), description)
            }
            Message::Warning(description) => {
                write!(f, "{}{} {}", Message::spaces(WARNING_LABEL), WARNING_LABEL.yellow().bold(), description)
            }
            Message::NoAction(message) => {
                write!(f, "{} {}", " ".repeat(ACTION_AREA), message)
            }
        }
    }
}
