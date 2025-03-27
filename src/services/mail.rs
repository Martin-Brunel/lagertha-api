use chrono::format::parse;
use chrono::{Datelike, Timelike, Utc};
use mail_send::{mail_builder::MessageBuilder, SmtpClientBuilder};
use rocket_dyn_templates::Template;
use std::{env, fs, time::Duration};
use woothee::parser::Parser;
pub struct MailService;

impl MailService {
    async fn send_email(message: MessageBuilder<'_>) {
        let smtp_address = match env::var("SMTP_HOST") {
            Ok(smtp_address) => smtp_address,
            Err(_) => return,
        };
        let smtp_port = match env::var("SMTP_PORT") {
            Ok(smtp_port) => match smtp_port.parse::<u16>(){
                Ok(port) => port,
                Err(_) => return
            },
            Err(_) => return,
        };
        let smtp_tls = env::var("SMTP_TLS")
            .unwrap_or_else(|_| format!("true"))
            .parse::<bool>()
            .unwrap();
        let smtp_creds_required = env::var("SMTP_CREDENTIALS_REQUIRED")
            .unwrap_or_else(|_| format!("false"))
            .parse::<bool>()
            .unwrap();

        let mut smtp_client = SmtpClientBuilder::new(smtp_address, smtp_port);

        if smtp_creds_required {
            let smtp_username = env::var("SMTP_USERNAME").unwrap();
            let smtp_password = env::var("SMTP_PASSWORD").unwrap();

            smtp_client = smtp_client.credentials((smtp_username, smtp_password));
        }

        smtp_client = smtp_client
            .implicit_tls(smtp_tls)
            .timeout(Duration::new(5, 0));
        let mut smtp_client = smtp_client.connect().await.unwrap();
        smtp_client.send(message).await.unwrap();
    }

    pub async fn send_validation_code(email: &String, login: &String, code: &String) {
        let template = fs::read_to_string("templates/validation.html")
            .unwrap()
            .replace("{{var:Code:\"\"}}", code);
        let smtp_from_name = env::var("SMTP_FROM_NAME").unwrap();
        let smtp_from = env::var("SMTP_FROM").unwrap();
        let login = login.clone();
        let email = email.clone();
        let message = MessageBuilder::new()
            .from((smtp_from_name.as_str(), smtp_from.as_str()))
            .to(vec![(login, email)])
            .subject("Hb Cyber_core: Complete Your Account Setup")
            .html_body(template);

        Self::send_email(message).await;
    }

    pub async fn send_forget_code(email: &String, login: &String, code: &String) {
        let template = fs::read_to_string("templates/forget.html")
            .unwrap()
            .replace("{{var:Code:\"\"}}", code);
        let smtp_from_name = env::var("SMTP_FROM_NAME").unwrap();
        let smtp_from = env::var("SMTP_FROM").unwrap();
        let login = login.clone();
        let email = email.clone();
        let message = MessageBuilder::new()
            .from((smtp_from_name.as_str(), smtp_from.as_str()))
            .to(vec![(login, email)])
            .subject("Hb Cyber_core: Your Password Reset Request")
            .html_body(template);

        Self::send_email(message).await;
    }

    pub async fn send_unfamiliar_connexion(
        email: &String,
        login: &String,
        ip: &String,
        user_agent: &String,
    ) {
        let now = Utc::now();

        let (is_pm, hour) = now.hour12();
        let (_, year) = now.year_ce();

        let date = format!(
            "{}-{:02}-{:02} {:02}:{:02}:{:02} {}",
            year,
            now.month(),
            now.day(),
            hour,
            now.minute(),
            now.second(),
            if is_pm { "PM" } else { "AM" }
        );

        let parser = Parser::new();
        let result = parser.parse(user_agent);

        // [Device Type], [Operating System], [Browser], [Browser Version]
        let device = match result {
            None => format!("-"),
            Some(res) => {
                format!("{}, {}, {}", res.category, res.os, res.name)
            }
        };

        let template = fs::read_to_string("templates/unfamiliar_connexion.html")
            .unwrap()
            .replace("{{var:Date:\"\"}}", &date)
            .replace("{{var:Login:\"\"}}", login)
            .replace("{{var:Device:\"\"}}", &device)
            .replace("{{var:Ip:\"\"}}", ip);
        let smtp_from_name = env::var("SMTP_FROM_NAME").unwrap();
        let smtp_from = env::var("SMTP_FROM").unwrap();
        let login = login.clone();
        let email = email.clone();
        let message = MessageBuilder::new()
            .from((smtp_from_name.as_str(), smtp_from.as_str()))
            .to(vec![(login, email)])
            .subject("Hb Cyber_core: New Device Sign-in Detected")
            .html_body(template);

        Self::send_email(message).await;
    }
}
