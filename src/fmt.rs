use std::io::Write;

pub fn success(title: &str) -> String {
    let title_len = title.len();
    let spaces = " ".repeat(12 - title_len);
    format!("{}\x1B[32;1m{}\x1B[0m", spaces, title)
}

pub fn error() -> String {
    "\x1B[31;1merror\x1B[0m".to_string()
}

#[allow(unused)]
pub fn warning() -> String {
    "\x1B[33;1mwarning\x1B[0m".to_string()
}

pub fn info(title: &str) -> String {
    let title_len = title.len();
    let spaces = " ".repeat(12 - title_len);
    format!("{}\x1B[36;1m{}\x1B[0m", spaces, title)
}

pub struct Progress;

impl Progress {
    pub fn new(title: &str, message: String) -> Self {
        print!("{} {} ...", info(title), message);
        let _ = std::io::stdout().flush();

        Self
    }

    pub fn finish(self, title: &str, message: String) {
        print!("\r{}\r", " ".repeat(100));
        let _ = std::io::stdout().flush();
        println!("{} {}", success(title), message);
    }
}
