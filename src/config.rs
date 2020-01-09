use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Command {
  Bin(String),
  Script(Vec<String>),
}

#[derive(Deserialize, Debug)]
struct RepoConfig {
  name: String,
  dir:  String,
  cmd:  Command,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
  repos: Vec<RepoConfig>,
}

impl AppConfig {
  pub fn load() -> crate::Result<Self> {
    let file_content = std::fs::read_to_string("./whl.yml")?;
    let config = serde_yaml::from_str(&file_content)?;
    Ok(config)
  }
}
