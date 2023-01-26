FROM rust:slim
WORKDIR /app
COPY . /app
RUN cargo build --release
EXPOSE 8080

ENTRYPOINT ["./target/release/autocomplete-api-poc"]