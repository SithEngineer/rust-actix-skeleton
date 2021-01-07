# Skeleton of an API written in Rust

This code sample only has an API with an endpoint: `/health-check` that returns a JSON object. 

Dependencies:
* Actix
* Serde

The server port can be set in the environment variable `SERVER_PORT`.

## How to build and run this project locally

Use `cargo watch -x run` to continuously build and run on changes saved to file.
Using Intellij IDEA add a new `Cargo` configuration and use `watch -x run` as the command.
The server is available in address `127.0.0.1:9090`.

## How to build and run this project in Docker
Use `docker-compose up` and the server will be available in address `127.0.0.1:9090`.