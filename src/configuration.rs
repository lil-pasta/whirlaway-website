use serde_aux::field_attributes::deserialize_number_from_string;
use std::convert::{TryFrom, TryInto};

pub enum Environment {
    Local,
    Prod,
}

impl Environment {
    pub fn as_string(&self) -> String {
        match self {
            Self::Local => "local".to_string(),
            Self::Prod => "prod".to_string(),
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "production" => Ok(Self::Prod),
            "local" => Ok(Self::Local),
            _ => Err(format!(
                "{s} is an invalid environment, use either 'production' or 'local'"
            )),
        }
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Settings {
    pub application: ApplicationSettings,
}

impl Settings {
    pub fn application_address(&self) -> String {
        format!("{}:{}", self.application.host, self.application.port)
    }
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct ApplicationSettings {
    host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    port: u16,
}

pub fn get_conf() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("failed to get current directory");
    let conf_path = base_path.join("conf");

    // get app environment from environment variable APP_ENVIRONMENT
    // defaults to "local"
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("problem getting app environment");
    // sets conf_path variable to point towards the proper environment
    // yaml
    let env_conf_path = conf_path.join(environment.as_string());

    // start layering on the configurations
    let settings = config::Config::builder()
        .add_source(config::File::from(conf_path.join("base")).required(true))
        .add_source(config::File::from(env_conf_path).required(true))
        .add_source(config::Environment::with_prefix("app").separator("__"))
        .build()?;

    settings.try_deserialize::<Settings>()
}
