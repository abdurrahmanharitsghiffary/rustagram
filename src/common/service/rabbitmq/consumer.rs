use crate::common::service::email::send_email;

use super::{consumer_tag::ConsumerTag, queue_name::QueueName};
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions},
    types::FieldTable,
};
use tokio_stream::StreamExt;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct EmailTask {
    pub to: String,
    pub subject: String,
    pub body: String,
}

pub async fn email_consumer(channel: lapin::Channel) {
    let mut consumer = channel
        .basic_consume(
            &QueueName::EmailQueue.value(),
            &ConsumerTag::EmailConsumer.value(),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Failed to create Email Consumer");

    while let Some(delivery) = consumer.next().await {
        let delivery = delivery.expect("Error while consuming message");

        let email_task: EmailTask =
            serde_json::from_slice(&delivery.data).expect("Failed to Serialize EmailTask");

        match send_email(&email_task.to, &email_task.subject, &email_task.body) {
            Err(err) => {
                log::error!("Failed sending email to {} : {}", &email_task.to, err);
            }
            Ok(_) => {
                log::info!("Email successfully sent to: {}", &email_task.to);
            }
        };

        delivery
            .ack(BasicAckOptions::default())
            .await
            .expect("Failed to acknowledge Message");
    }
}
