use config::{Config, Environment, File};
use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub database: DatabaseConfig,
    #[serde(default = "Default::default")]
    pub log: LogConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub filename: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct LogConfig {
    pub log_path: String,
    pub name_prefix: String,
    pub file_log_enable: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            log_path: "logs".to_string(),
            name_prefix: "note-backend.log".to_string(),
            file_log_enable: false,
        }
    }
}

pub fn load_config() -> Result<Settings, config::ConfigError> {
    Ok(Config::builder()
        .add_source(File::with_name("config/default"))
        .add_source(File::with_name("config/local").required(false))
        .add_source(Environment::with_prefix("APP"))
        .build()?
        .try_deserialize()?)
}

pub static SETTINGS: OnceLock<Settings> = OnceLock::new();

pub fn init_config() -> Result<(), config::ConfigError> {
    let settings = load_config()?;
    SETTINGS.set(settings).expect("配置初始化失败");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn database_config_deserializes_filename() {
        let json = r#"{"filename":"note.db"}"#;
        let cfg: DatabaseConfig = serde_json::from_str(json).unwrap();
        assert_eq!(cfg.filename, "note.db");
    }

    #[test]
    fn database_config_has_no_url_field() {
        let json = r#"{"filename":"x.db"}"#;
        let cfg: DatabaseConfig = serde_json::from_str(json).unwrap();
        assert_eq!(cfg.filename, "x.db");
    }
}
