#[derive(Clone)]
pub enum Edition {
    Edition2026,
}

impl Edition {
    pub fn parse_edition(toml_table: &toml::Table) -> Result<Self, String> {
        let edition_str = toml_table
            .get("edition")
            .and_then(|v| v.as_str())
            .unwrap_or("2026");

        match edition_str {
            "2026" => Ok(Edition::Edition2026),
            _ => Err(format!("Unsupported edition: {}", edition_str)),
        }
    }
}
