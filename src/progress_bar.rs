use std::io;
use std::io::Write;
use std::thread::current;
use crate::message::Message;

pub struct ProgressBar {
    total: usize,
    current: usize,
    message: String,
    print_en: bool,
}

impl ProgressBar {
    pub fn new(message: &str, total: usize, print_en: bool) -> Self {
        ProgressBar {
            total,
            current: 0,
            message: message.to_string(),
            print_en,
        }
    }

    pub fn print(&self) {
        let cmd = Message::in_progress(&self.message, self.current, self.total);

        if self.current >= self.total {
            if self.print_en {
                print!("{}\r", " ".repeat(cmd.to_string().len()));
                io::stdout().flush().unwrap();
            }
        } else {
            if self.print_en {
                print!("{}\r", cmd);
                io::stdout().flush().unwrap();
            }
        }
    }
}

impl Iterator for ProgressBar {
    type Item = Message;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;

        self.print();

        if self.current >= self.total {
            None
        } else {
            Some(Message::in_progress(&self.message, self.current, self.total))
        }
    }
}
