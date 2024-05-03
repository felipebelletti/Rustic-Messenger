# RusticMessenger

RusticMessenger is an asynchronous messaging application built with Rust, using RabbitMQ for message brokering.

It features a command-line interface for sending and receiving messages.

## Features

- **Asynchronous Messaging**: Built on top of Rust's async/await paradigm.
- **CLI Support**: Command-line interface to send and receive messages interactively.
- **Customizable**: Supports custom queue names as well predefined settings via `config.toml`.

### Prerequisites

- Rust/Cargo
- RabbitMQ server

### Installation

1. Clone the repository:
```
git clone https://github.com/yourgithub/RusticMessenger.git
cd RusticMessenger
cargo build --release
```
2. Build using Cargo:
```
cargo build --release
```

### Usage
#### Sending a message:
```
cargo run -- send -m "test message"
```

#### Listening for messages:
```
cargo run -- receive
```

### Command-Line Arguments
* -q, --queuename: Specifies a custom queue name for sending or receiving messages.
* -m, --message: Specifies the content of the message to be sent to the queue (required for `send` subcommand).