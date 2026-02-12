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
pub fn info(title: &str) -> String {
    let title_len = title.len();
    let spaces = " ".repeat(12 - title_len);
    format!("{}\x1B[36;1m{}\x1B[0m", spaces, title)
}

pub struct Progress {
    title: String,
    total: usize,
    current: usize,
}

impl Progress {
    pub fn new(title: &str, total: usize) -> Self {
        print!("{} [>{}] 0/{}", info(title), " ".repeat(25), total);
        let _ = std::io::stdout().flush();

        Self {
            title: title.to_string(),
            total,
            current: 0,
        }
    }

    fn clear_line() {
        print!("\r\x1B[K");
    }

    pub fn next(&mut self, message: &str) {
        Self::clear_line();
        println!("{} ", message);

        self.current += 1;
        let percentage = self.current as f64 / self.total as f64;
        let filled_length = (percentage * 25.0).round() as usize;
        let bar = format!(
            "[{}>{}] {}/{}",
            "=".repeat(filled_length),
            " ".repeat(25 - filled_length),
            self.current,
            self.total
        );
        print!("\r{} {}", info(&self.title), bar);
        let _ = std::io::stdout().flush();
    }

    pub fn finish(self) {
        Self::clear_line();
    }
}
