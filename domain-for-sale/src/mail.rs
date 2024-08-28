use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};

use crate::{config, Error, Result};

pub fn send(cfg: &config::MailConfig, subject: &str, body: String) -> Result<()> {
    let email = Message::builder()
        .from(cfg.address.as_str().parse().unwrap())
        .to(cfg.to.as_str().parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body)?;

    let creds = Credentials::new(cfg.address.clone(), cfg.password.clone());

    let mailer = SmtpTransport::relay(&cfg.server)?
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::from(e)),
    }
}
