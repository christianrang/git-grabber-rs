use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    #[serde(rename = "ssh")]
    pub ssh_key: Option<SshKeyConfiguration>,
    pub git: GitConfiguration,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SshKeyConfiguration {
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GitConfiguration {
    #[serde(rename = "repos")]
    pub repositories: Vec<RepositoryConfiguration>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepositoryConfiguration {
    #[serde(rename = "target_dir")]
    pub target_directory: Box<String>,
    pub ssh_url: String,
    pub branches: Vec<String>,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration {
            ssh_key: None,
            git: GitConfiguration {
                repositories: Vec::new(),
            },
        }
    }
}

pub fn get_configuration(file_path: &Path) -> Result<Configuration, Box<dyn Error>> {
    let file = std::fs::File::open(file_path)?;
    let reader = std::io::BufReader::new(file);
    let configuration: Configuration = serde_yaml::from_reader(reader)?;
    return Ok(configuration);
}
