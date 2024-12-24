use deadpool_lapin::Pool;
use lapin::{
    options::BasicPublishOptions, publisher_confirm::PublisherConfirm, BasicProperties, Error,
};

use super::{channel::create_rabbitmq_channel, consumer::EmailTask, queue_name::QueueName};

pub async fn publish_email_task(
    name: QueueName,
    payload: EmailTask,
    pool: &Pool,
) -> Result<PublisherConfirm, Error> {
    let channel = create_rabbitmq_channel(&[QueueName::EmailQueue], &pool).await;

    let email_task = serde_json::to_vec(&payload).expect("Failed to serialize email task");

    channel
        .basic_publish(
            "",
            &name.value(),
            BasicPublishOptions::default(),
            &email_task,
            BasicProperties::default(),
        )
        .await
}
