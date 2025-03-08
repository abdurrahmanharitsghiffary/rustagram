use crate::common::typings::enums::AppEnvirontment;
use config::{Config, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub user: String,
    pub password: String,
    pub db: String,
    pub host: String,
    pub port: u64,
    pub pool_size: u32,
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        Self {
            user: "postgres".to_string(),
            password: "postgres".to_string(),
            db: "postgres".to_string(),
            host: "localhost".to_string(),
            port: 5432,
            pool_size: 10,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct RabbitMQSettings {
    pub default_user: String,
    pub default_pass: String,
    pub default_vhost: String,
    pub node_ip_address: String,
    pub node_port: u64,
    pub management_bind_ip: String,
    pub pool_size: usize,
}

impl Default for RabbitMQSettings {
    fn default() -> Self {
        Self {
            default_user: "guest".to_string(),
            default_pass: "guest".to_string(),
            default_vhost: "/".to_string(),
            node_ip_address: "127.0.0.1".to_string(),
            node_port: 5672,
            management_bind_ip: "127.0.0.1".to_string(),
            pool_size: 10,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct SMTPSettings {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppSettings {
    pub database_url: String,
    pub rabbitmq_url: String,
    pub debug: bool,
    pub environtment: AppEnvirontment,
    pub jwt_secret: String,
    pub database: DatabaseSettings,
    pub rabbitmq: RabbitMQSettings,
    pub smtp: SMTPSettings,
}

impl AppSettings {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();

        let mut settings: AppSettings = Config::builder()
            .add_source(
                Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(" "),
            )
            .build()
            .expect("Failed to load the Config.")
            .try_deserialize()
            .expect("Failed to deserialize Config into settings.");

        settings.database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            settings.database.user,
            settings.database.password,
            settings.database.host,
            settings.database.port,
            settings.database.db
        );

        settings.rabbitmq_url = format!(
            "amqp://{}:{}@{}:{}/{}",
            settings.rabbitmq.default_user,
            settings.rabbitmq.default_pass,
            settings.rabbitmq.node_ip_address,
            settings.rabbitmq.node_port,
            settings.rabbitmq.default_vhost
        );

        settings
    }
}
