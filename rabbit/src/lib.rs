use lapin::{
    options::*, publisher_confirm::Confirmation, types::FieldTable, BasicProperties, Connection,
    ConnectionProperties, Result,
};
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio_amqp::*;
use tracing;
use tracing::{info, Level};
use tracing_subscriber;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
pub fn execute() {
    let collector = tracing_subscriber::fmt()
        // filter spans/events with level TRACE or higher.
        .with_max_level(Level::TRACE)
        // build but do not install the subscriber.
        .finish();
    tracing_subscriber::fmt::init();
    let rt = Arc::new(Runtime::new().expect("failed to create runtime"));
    rt.block_on(tokio_main(rt.clone())).expect("error");
}
async fn tokio_main(_rt: Arc<Runtime>) -> Result<()> {
    info!("tokio main ");
    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());
    let conn = Connection::connect(&addr, ConnectionProperties::default().with_tokio())
        .await
        .expect("connect error"); // Note the `with_tokio()` here
    info!("tokio after conn");
    // Rest of your program
    let channel_a = conn.create_channel().await?;
    let channel_b = conn.create_channel().await?;
    //channel_b.basic_qos(3, BasicQosOptions::default());
    //channel_a.basic_qos(3, BasicQosOptions::default());
    let queue = channel_a
        .queue_declare(
            "hello",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    info!(?queue, "Declared queue");

    let mut consumer = channel_a
        .basic_consume(
            "hello",
            "my_consumer1",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("basic_consume");
    let mut consumer1 = channel_b
        .basic_consume(
            "hello",
            "my_consumer2",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("basic_consume");

    let thread = tokio::spawn(async move {
        let mut consume_iter = consumer.into_iter();
        while let Some(delivery) = consume_iter.next() {
            let (_, delivery) = delivery.expect("error in consumer");
            let data = delivery.data.clone();
            info!("consumer 1 Delivery: {:?}", String::from_utf8_lossy(&data));
            delivery.ack(BasicAckOptions::default()).await.expect("ack");
        }
    });

    let thread1 = tokio::spawn(async move {
        let mut consume_iter = consumer1.into_iter();
        while let Some(delivery) = consume_iter.next() {
            let (_, delivery) = delivery.expect("error in consumer");
            let data = delivery.data.clone();
            info!("consumer 2 Delivery: {:?}", String::from_utf8_lossy(&data));
            delivery.ack(BasicAckOptions::default()).await.expect("ack");
        }
    });

    thread.await.expect("await thread");
    thread1.await.expect("await thread");

    // loop {
    //     let confirm = channel_a
    //         .basic_publish(
    //             "",
    //             "hello",
    //             BasicPublishOptions::default(),
    //             payload.to_vec(),
    //             BasicProperties::default(),
    //         )
    //         .await?
    //         .await?;
    //     assert_eq!(confirm, Confirmation::NotRequested);
    // }
    Ok(())
}
