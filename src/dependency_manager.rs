use std::path::PathBuf;

use dirs::config_dir;

use crate::{
    dependency::Dependency,
    git::{git_checkout, git_clone},
    package::Package,
};

fn dependency_storage_path(dependency: &Dependency) -> PathBuf {
    let pkg_storage_fullpath = config_dir()
        .expect("Failed to get config directory")
        .join("tailor")
        .join("packages");

    match dependency {
        Dependency::Local { path, .. } => path.clone(),
        Dependency::Registry { name, version } => {
            pkg_storage_fullpath.join(format!("{name}@{version}"))
        }
        Dependency::Git { name, revision, .. } => {
            pkg_storage_fullpath.join(format!("{name}@{revision}"))
        }
    }
}

fn dependency_is_valid(dependency: &Dependency) -> bool {
    dependency_storage_path(dependency)
        .join("Tailor.toml")
        .exists()
}

fn fetch_git_url(name: &str, version: &str) -> Result<String, String> {
    let url = format!(
        "http://127.0.0.1:5000/registry?name={}&version={}",
        name, version
    );

    reqwest::blocking::get(url)
        .map_err(|_| "Failed to fetch URL".to_string())?
        .text()
        .map_err(|_| "Failed to fetch URL".to_string())
}

fn download_git_dependency(name: &str, url: &str, revision: &str) -> Result<(), String> {
    let dep_path = dependency_storage_path(&Dependency::Git {
        name: name.to_string(),
        url: url.to_string(),
        revision: revision.to_string(),
    });

    git_clone(url, &dep_path)?;

    git_checkout(revision, &dep_path)?;

    Ok(())
}

fn dependency_download(dependency: &Dependency) -> Result<(), String> {
    match dependency {
        Dependency::Local { .. } => Ok(()),
        Dependency::Registry { name, version } => {
            let url = fetch_git_url(name, version)?;
            download_git_dependency(name, &url, version)
        }
        Dependency::Git {
            name,
            url,
            revision,
        } => download_git_dependency(name, url, revision),
    }
}

fn dependency_source_paths(dependency: &Dependency) -> Result<Vec<String>, String> {
    let dep_path = dependency_storage_path(dependency);
    let manifest_path = dep_path.join("Tailor.toml");

    Ok(Package::from_file(&manifest_path)?
        .sources()
        .iter()
        .map(|source| dep_path.join(source).to_string_lossy().to_string())
        .collect())
}

fn dependency_include_paths(dependency: &Dependency) -> Result<Vec<String>, String> {
    let dep_path = dependency_storage_path(dependency);
    let manifest_path = dep_path.join("Tailor.toml");

    Ok(Package::from_file(&manifest_path)?
        .includes()
        .iter()
        .map(|include| dep_path.join(include).to_string_lossy().to_string())
        .collect())
}

pub fn resolve_dependencies(pkg: &Package) -> Result<(Vec<String>, Vec<String>), String> {
    for dependency in pkg.dependencies() {
        if dependency_is_valid(dependency) {
            continue;
        }

        dependency_download(dependency)?;
    }

    let sources = pkg
        .dependencies()
        .iter()
        .map(|dep| dependency_source_paths(dep).unwrap_or(vec![]))
        .flatten()
        .collect();
    let includes = pkg
        .dependencies()
        .iter()
        .map(|dep| dependency_include_paths(dep).unwrap_or(vec![]))
        .flatten()
        .collect();

    Ok((sources, includes))
}
