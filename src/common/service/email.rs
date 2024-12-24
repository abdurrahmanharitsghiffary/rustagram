use std::env;

use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};

pub fn send_email(to: &'static str, subject: &'static str, body: String) -> Result<(), String> {
    let email = match Message::builder()
        .from(
            "Rustagram <rustagram124@gmail.com>"
                .parse()
                .expect("Failed to parse [email.from]"),
        )
        .to(to.parse().expect("Failed to parse [email.to]"))
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(body)
    {
        Ok(message) => message,
        Err(e) => return Err(format!("Could not send email: {e:?}")),
    };

    let creds = Credentials::new(
        env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set"),
        env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set"),
    );

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .expect("Failed to build the SMTP Transport")
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Could not send email: {e:?}")),
    }
}
