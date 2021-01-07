FROM rust:1.49
WORKDIR /usr/src/server
COPY . .
RUN cargo install --path .
ENV SERVER_PORT=8080
EXPOSE 8080
CMD ["rust-actix-skeleton"]