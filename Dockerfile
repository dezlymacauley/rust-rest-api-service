# Stage 1 - Builder
FROM rust:1.93.0 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

#______________________________________________________________________________

# Stage 2
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/rust-rest-api-service .
EXPOSE 8000
CMD ["./rust-rest-api-service"]

#______________________________________________________________________________
