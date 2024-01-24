#[macro_use] 
extern crate rocket;

use rocket::serde::{json::Json};

mod templates {
    pub mod helpers {
        pub mod generate_recovery_template;
    }
}

mod services {
    pub mod email_service;
}

#[derive(Debug, serde::Deserialize, rocket::FromForm)]
struct RecoveryPasswordData {
    username: String,
    url: String,
    email: String
}

#[post("/recovery-password", data = "<recovery_password_data>")]
fn recovery_password(recovery_password_data: Json<RecoveryPasswordData>) -> String {
    let smtp_transport = services::email_service::start_email_service();
    let username = &recovery_password_data.username;
    let url = &recovery_password_data.url;
    let email = &recovery_password_data.email;

    let template = templates::helpers::generate_recovery_template::generate_recovery_html(username.as_str(), url.as_str());
    match services::email_service::send_email(smtp_transport, template, email.to_string()) {
        Ok(_) => format!("Email send correctly."),
        Err(e) => format!("Error on send email: {}", e),
    }
}

#[get("/")]
fn hello() -> &'static str {
    "Server running 🐱 - 📬 - 🦀!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello, recovery_password])
}