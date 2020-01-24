use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Command {
  Bin(String),
  Script(Vec<String>),
}

#[derive(Deserialize, Debug)]
pub struct RepoConfig {
  pub name: String,
  pub dir:  String,
  pub cmd:  Command,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
  pub repos: Vec<RepoConfig>,
}

impl AppConfig {
  pub fn load() -> crate::Result<Self> {
    let file_content = std::fs::read_to_string("./whl.yml")?;
    let config = serde_yaml::from_str(&file_content)?;
    Ok(config)
  }
}
