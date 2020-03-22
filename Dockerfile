FROM rustlang/rust:nightly-buster-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /app/target/release/img-transformer /app/img-transformer
CMD ./img-transformer
