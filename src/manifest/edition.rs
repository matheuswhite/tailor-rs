#[derive(Clone)]
pub enum Edition {
    Edition2025,
}

impl Edition {
    pub fn parse_edition(toml_table: &toml::Table) -> Result<Self, String> {
        let edition_str = toml_table
            .get("edition")
            .and_then(|v| v.as_str())
            .unwrap_or("2025");

        match edition_str {
            "2025" => Ok(Edition::Edition2025),
            _ => Err(format!("Unsupported edition: {}", edition_str)),
        }
    }
}
