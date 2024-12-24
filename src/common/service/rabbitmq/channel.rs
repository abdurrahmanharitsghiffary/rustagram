use deadpool_lapin::Pool;
use lapin::{options::QueueDeclareOptions, types::FieldTable, Channel};

use super::queue_name::QueueName;

pub async fn create_rabbitmq_channel(queue_names: &[QueueName], pool: &Pool) -> Channel {
    let connection = pool
        .get()
        .await
        .expect("Failed to get RabbitMQ pool connection");

    let channel = connection
        .create_channel()
        .await
        .expect("Failed to create RabbitMQ channel");

    for (_, queue_name) in queue_names.into_iter().enumerate() {
        channel
            .queue_declare(
                &queue_name.value(),
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect(&format!("Failed to declare {}", queue_name.value()));
    }

    channel
}
