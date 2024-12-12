FROM rust:latest

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN cargo fetch

COPY . .

RUN cargo build --release

EXPOSE 3000

ENTRYPOINT ["/app/target/release/databases-performance-test"]
CMD ["--default-arguments"] 
