# A Pub/Sub Broker example in Lunatic
This project leverages the `lunatic-rs` crate to create a supervised, actor-based system in Rust. It implements an intermediary broker process that manages subscriptions and relays messages between publishers and subscribers. The broker facilitates a publisher/subscriber pattern, where multiple subscribing processes can subscribe to specific topics. Publishers send messages to the broker, which then forwards these messages to all subscribers of the relevant topic. The broker is designed to be supervised, ensuring resilience and fault tolerance by restarting failed processes. This project demonstrates how to use `lunatic-rs` to build reliable and scalable communication systems in Rust.

## Getting started

you can build this project to a WASM binary using the following cmd, ensure you have the <> target configured and the lunatic runtime installed

`cargo build --release --target=wasm32-wasi`

You can run the project using the lunatic runtime with this command from the root of the project
`lunatic run target/wasm32-wasi/release/lunatic-ex.wasm`

