use std::env;
use dotenv::dotenv;

use lettre::{transport::smtp::authentication::Credentials, SmtpTransport, Transport, };
use lettre::message::{header::ContentType, Message, Mailbox};

pub fn start_email_service() -> SmtpTransport {
    dotenv().ok();

    let smtp_server = env::var("SMTP_SERVER").unwrap_or_else(|_| "smtp.example.com".to_string());
    let smtp_port = env::var("SMTP_PORT").unwrap_or_else(|_| "587".to_string());
    let smtp_username = env::var("SMTP_USERNAME").unwrap_or_else(|_| "your_username".to_string());
    let smtp_password = env::var("SMTP_PASSWORD").unwrap_or_else(|_| "your_password".to_string());

    let smtp_credentials = Credentials::new(smtp_username.clone(), smtp_password.clone());
    let smtp_transport = SmtpTransport::relay(&smtp_server)
        .unwrap()
        .port(smtp_port.parse().unwrap())
        .credentials(smtp_credentials)
        .build();
    
    smtp_transport
}

pub fn send_email(smtp_transport: SmtpTransport, template: String) ->  Result<lettre::transport::smtp::response::Response, lettre::transport::smtp::Error> {
    dotenv().ok();

    // Configuración del correo electrónico
    let from_address = env::var("FROM_ADDRESS").unwrap_or_else(|_| "your_email@example.com".to_string());
    let to_address = env::var("TO_ADDRESS").unwrap_or_else(|_| "recipient@example.com".to_string());
    
    // Parse email addresses and panic if parsing fails
    let to_mailbox: Mailbox = to_address.parse().expect("Failed to parse TO_ADDRESS");
    let from_mailbox: Mailbox = from_address.parse().expect("Failed to parse FROM_ADDRESS");

        
    let m = Message::builder()
    .to(to_mailbox)
    .from(from_mailbox)
    .subject("Recuperación de contraseña en Nima.cl")
    .header(ContentType::TEXT_HTML)
    .body(String::from(template));

    smtp_transport.send(&m.unwrap())
}