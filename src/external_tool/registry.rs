use crate::config::Config;

pub struct Registry {
    config: Config,
}

impl Registry {
    pub fn package_url(&self, name: &str, version: &str) -> Result<String, String> {
        let url = format!(
            "{}/packages/{}/{}-{}.tar.gz",
            self.config.registry_url, name, name, version
        );

        let response = reqwest::blocking::get(url)
            .map_err(|err| format!("Failed to get package {}@{} git URL: {err}", name, version))?;
        let rsp_text = response.text().map_err(|err| {
            format!(
                "Failed to read response text for package {}@{}: {err}",
                name, version
            )
        })?;
        Ok(rsp_text)
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self {
            config: Config::load(),
        }
    }
}
