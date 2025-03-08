use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};

use crate::config::settings::SMTPSettings;

pub fn send_email(
    settings: &SMTPSettings,
    to: &str,
    subject: &str,
    body: &str,
) -> Result<(), String> {
    let email = match Message::builder()
        .from(
            "Rustagram <rustagram124@gmail.com>"
                .parse()
                .expect("Failed to parse [email.from]"),
        )
        .to(to.parse().expect("Failed to parse [email.to]"))
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(body.to_string())
    {
        Ok(message) => message,
        Err(e) => return Err(format!("Could not send email: {e:?}")),
    };

    let creds = Credentials::new(settings.username.clone(), settings.password.clone());

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .expect("Failed to build the SMTP Transport")
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Could not send email: {e:?}")),
    }
}
