use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RepoConfig {
  pub name:   String,
  pub dir:    Option<String>,
  pub cmd:    String,
  #[serde(rename = "ref")]
  pub branch: String,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
  pub repos: Vec<RepoConfig>,
}

impl AppConfig {
  pub fn load() -> crate::Result<Self> {
    let file_content = std::fs::read_to_string("./config/whl.yml")?;
    let config = serde_yaml::from_str(&file_content)?;
    Ok(config)
  }
}
