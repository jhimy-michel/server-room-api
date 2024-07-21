# Server Room Temperature Monitoring API

This project is a Rust-based web server that simulates real-time temperature monitoring of server racks in a server room. It utilizes the Actix-web framework and provides endpoints for starting, stopping, and streaming server room temperature data.

## Features

- **Real-Time Data Streaming**: Stream server room temperature data in real-time.
- **Control Streaming**: Start and stop the data stream with HTTP requests.
- **CORS Support**: Enabled Cross-Origin Resource Sharing (CORS) for seamless integration with front-end applications.

## Endpoints

- `GET /server-room-stream`: Streams server room temperature data.
- `POST /start`: Starts the data stream.
- `POST /stop`: Stops the data stream.

## Server Room Data Format

The server room data contains a timestamp and an array of rack temperature readings. Each reading includes:

- `id`: Identifier of the rack.
- `temperature`: Temperature of the rack.
- `status`: Temperature status (`Too Cold`, `Too Hot`, `Optimal`, `Acceptable`).

## Prerequisites

- **Rust**: Install Rust from [rust-lang.org](https://www.rust-lang.org/).
- **Docker**: Install Docker from [docker.com](https://www.docker.com/).

## Getting Started

### Clone the Repository

```sh
git clone https://github.com/yourusername/server-room-monitoring.git
cd server-room-monitoring
```

### Build and Run the Server
You can build and run the server using Cargo:

```sh
Copy code
cargo build --release
cargo run --release
```

The server will be running at http://127.0.0.1:8080.

## Example Usage
Start Streaming

```sh
curl -X POST http://127.0.0.1:8080/start
```

Stop Streaming
```sh
curl -X POST http://127.0.0.1:8080/stop
```

### Stream Server Room Data

Open a browser or use curl to connect to the stream:

```sh

curl http://127.0.0.1:8080/server-room-stream
```

## License
This project is licensed under the MIT License. 

## Acknowledgements
* Actix-web for providing a powerful web framework.
* Chrono for date and time handling.
* Serde for serialization and deserialization.
* Tokio for asynchronous runtime.