FROM rust

COPY . .

ENV HOST 0.0.0.0

EXPOSE 8080:8080

RUN cargo build --release

CMD ["./target/release/real-time-data-api", "--port", "$PORT"]