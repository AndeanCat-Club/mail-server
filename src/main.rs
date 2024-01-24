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
}

#[post("/recovery-password", data = "<recovery_password_data>")]
fn recovery_password(recovery_password_data: Json<RecoveryPasswordData>) -> String {
    let smtp_transport = services::email_service::start_email_service();
    let username = &recovery_password_data.username;
    let url = &recovery_password_data.url;
    let template = templates::helpers::generate_recovery_template::generate_recovery_html(username.as_str(), url.as_str());
    match services::email_service::send_email(smtp_transport, template) {
        Ok(_) => format!("Correo electrónico enviado correctamente."),
        Err(e) => format!("Error al enviar el correo electrónico: {}", e),
    }
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello, recovery_password])
}