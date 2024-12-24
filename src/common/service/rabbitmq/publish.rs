use deadpool_lapin::Pool;
use lapin::{
    options::BasicPublishOptions, publisher_confirm::PublisherConfirm, BasicProperties, Error,
};

use super::{channel::create_rabbitmq_channel, queue_name::QueueName};

pub async fn publish_task(
    name: QueueName,
    payload: &[u8],
    pool: &Pool,
) -> Result<PublisherConfirm, Error> {
    let channel = create_rabbitmq_channel(&[QueueName::EmailQueue], &pool).await;

    channel
        .basic_publish(
            "",
            &name.value(),
            BasicPublishOptions::default(),
            &payload,
            BasicProperties::default(),
        )
        .await
}
