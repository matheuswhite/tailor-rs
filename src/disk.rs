use std::fs::File;

pub struct Disk {
    root: String,
}

impl Disk {
    pub fn new(project_name: &str) -> Self {
        Disk {
            root: project_name.to_string() + "/",
        }
    }

    pub fn create_store(&self, path: &str, content: String) {
        let mut file = File::create(self.root.clone() + path).unwrap();
        std::io::copy(&mut content.as_bytes(), &mut file).unwrap();
    }
}
