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

    pub fn create_store(&self, path: &str, content: String) -> anyhow::Result<()> {
        let mut file = File::create(self.root.clone() + path)?;
        std::io::copy(&mut content.as_bytes(), &mut file)?;

        Ok(())
    }
}
