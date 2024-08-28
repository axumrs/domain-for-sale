use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HCaptchaConfig {
    pub site_key: String,
    pub secret_key: String,
    pub timeout: i64,
}

#[derive(Debug, Deserialize)]
pub struct MailConfig {
    pub address: String,
    pub password: String,
    pub server: String,
    pub to: String,
    pub timeout: i64,
}

#[derive(Debug, Deserialize)]
pub struct WebConfig {
    pub address: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub hcaptcha: HCaptchaConfig,
    pub mail: MailConfig,
    pub web: WebConfig,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()?
            .try_deserialize()
    }
}
