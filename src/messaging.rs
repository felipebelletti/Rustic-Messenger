use futures_lite::stream::StreamExt;
use lapin::{
    options::*, publisher_confirm::PublisherConfirm, types::FieldTable, BasicProperties, Channel,
};
use log::{error, info};

pub async fn send_message(
    channel: &Channel,
    queue_name: &str,
    message: &str,
) -> Result<PublisherConfirm, lapin::Error> {
    let payload = message.as_bytes().to_vec();
    channel
        .basic_publish(
            "",
            queue_name,
            BasicPublishOptions::default(),
            &payload,
            BasicProperties::default(),
        )
        .await
}

pub async fn message_receiver(channel: Channel, queue_name: String) -> Result<(), lapin::Error> {
    let mut consumer = channel
        .basic_consume(
            &queue_name,
            "consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    info!("Consumer created for queue '{}'", queue_name);

    while let Some(delivery_result) = consumer.next().await {
        let delivery = delivery_result?;
        match std::str::from_utf8(&delivery.data) {
            Ok(data) => {
                info!("Received message: {}", data);
                delivery.ack(BasicAckOptions::default()).await?
            }
            Err(err) => {
                error!("Error parsing message data: {}", err);
                continue;
            },
        }
    }

    Ok(())
}
