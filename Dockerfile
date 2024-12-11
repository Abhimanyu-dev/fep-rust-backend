FROM rust:1 AS chef 
RUN cargo install cargo-chef 
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin fep-rust-backend

FROM debian:bookworm-slim AS runtime
WORKDIR app
COPY --from=builder /app/target/release/fep-rust-backend /usr/local/bin
ENTRYPOINT ["/usr/local/bin/fep-rust-backend"]
