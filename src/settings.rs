#[derive(serde::Deserialize, Debug, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub target: TargetSettings,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct TargetSettings {
    pub address: String,
}

enum Environment {
    Local,
    Production,
}
impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}
impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Unable to determine current directory.");
    let conf_path = base_path.join("configuration");

    let env: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    let settings = config::Config::builder()
        .add_source(config::File::from(conf_path.join("base.yml")))
        .add_source(config::File::from(
                conf_path.join(format!("{}.yml", env.as_str())),
                ))
        .add_source(config::Environment::with_prefix("APP").prefix_separator("_").separator("__"),).build()?;

    settings.try_deserialize()
}
