FROM rust:latest

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN cargo fetch

COPY . .

RUN cargo build --release

EXPOSE 3000

CMD ["/app/target/release/databases-performance-test"]
