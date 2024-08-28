use serde::{Deserialize, Serialize};

use crate::{config, Result};

#[derive(Deserialize)]
pub struct VerifyResponse {
    pub success: bool,
}

#[derive(Serialize)]
pub struct VerifyRequest<'a> {
    pub secret: &'a str,
    pub response: &'a str,
}

pub async fn verify(cfg: &config::HCaptchaConfig, token: &str) -> Result<bool> {
    let cli = reqwest::ClientBuilder::new()
        .connect_timeout(std::time::Duration::from_secs(cfg.timeout as u64))
        .build()?;
    let form = VerifyRequest {
        secret: &cfg.secret_key,
        response: token,
    };
    let res = cli
        .post("https://hcaptcha.com/siteverify")
        .form(&form)
        .send()
        .await?;
    let res: VerifyResponse = res.json().await?;
    Ok(res.success)
}
