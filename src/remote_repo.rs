use crate::disk::Disk;

pub struct Github {
    user: String,
    reposiroty: String,
    disk: Disk,
}

impl Github {
    const URL_PREFIX: &'static str = "https://raw.githubusercontent.com/";

    pub fn new(user: &str, repository: &str, disk: Disk) -> Self {
        Github {
            user: user.to_string(),
            reposiroty: repository.to_string(),
            disk,
        }
    }

    pub fn disk(&self) -> &Disk {
        &self.disk
    }

    pub async fn get(&self, path: &str) -> String {
        let url = Github::URL_PREFIX.to_string() + &self.user + "/" + &self.reposiroty + "/main/" + path;

        let rsp = reqwest::get(url).await.unwrap();
        rsp.text().await.unwrap()
    }

    pub async fn get_n_store(&self, remote_path: &str, local_path: &str, file: &str) {
        let remote_filepath = remote_path.to_string() + file;
        let local_filepath = local_path.to_string() + file;

        let content = self.get(&remote_filepath).await;
        self.disk.create_store(&local_filepath, content);
    }
}
