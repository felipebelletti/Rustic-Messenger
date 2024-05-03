use clap::{Arg, Command};
use config::Settings;
use env_logger::{Builder, Env};
use log::{error, info};
use std::error::Error;

mod config;
mod messaging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Builder::from_env(Env::default().default_filter_or("debug")).init();

    let matches = Command::new("RusticMessenger")
        .about("Asynchronous RabbitMQ Messenger")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("send")
                .short_flag('s')
                .long_flag("send")
                .about("Sends a message to a RabbitMQ queue")
                .arg(
                    Arg::new("message")
                        .help("Specifies the content of the message to be sent to the queue.")
                        .short('m')
                        .long("message")
                        .num_args(1)
                        .required(true),
                )
        )
        .subcommand(
            Command::new("receive")
                .short_flag('r')
                .long_flag("receive")
                .about("Starts listening for incoming messages on the configured RabbitMQ queue"),
        )
        .arg(
            Arg::new("queuename")
                .help("Specifies a custom queue name for sending or receiving messages. This overrides the default queue name set in the config.toml file.")
                .short('q')
                .long("queuename")
                .num_args(1)
                .required(false),
        )
        .get_matches();

    let Settings {
        mut queue_name,
        amqp_address,
    } = config::Settings::from_file("./config.toml")
        .map_err(|e| format!("Failed to load settings: {}", e))?;

    if let Some(forced_queue_value) = matches.get_one::<String>("queuename") {
        queue_name = forced_queue_value.clone();
    }

    let conn = lapin::Connection::connect(&amqp_address, Default::default())
        .await
        .map_err(|e| format!("Failed to connect to RabbitMQ: {}", e))?;

    let channel = conn
        .create_channel()
        .await
        .map_err(|e| format!("Failed to create RabbitMQ channel: {}", e))?;

    let queue = channel
        .queue_declare(
            &queue_name,
            lapin::options::QueueDeclareOptions::default(),
            lapin::types::FieldTable::default(),
        )
        .await
        .map_err(|e| format!("Failed to declare a queue: {}", e))?;

    match matches.subcommand() {
        Some(("send", send_matches)) => {
            let message = send_matches
                .get_one::<String>("message")
                .expect("clap required argument");

            match messaging::send_message(&channel, &queue_name, message).await {
                Ok(_) => info!("Successfully sent message to '{}'", &queue_name),
                Err(e) => error!("Failed to publish message to '{}': {}", &queue_name, e),
            };
        }
        Some(("receive", _)) => {
            let receiver_handler = tokio::spawn(messaging::message_receiver(
                channel.clone(),
                queue.name().to_string(),
            ));

            match receiver_handler.await {
                Ok(task_result) => {
                    task_result.map_err(|err| format!("Message receiver error: {}", err))?
                }
                Err(err) => Err(std::io::Error::new(std::io::ErrorKind::Other, err))?,
            }
        }
        _ => unreachable!(),
    }

    Ok(())
}
