FROM rust:slim-bullseye as chef
ENV SQLX_OFFLINE=true
RUN cargo install cargo-chef
WORKDIR /cochinillos-voladores

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /cochinillos-voladores/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN apt-get update \
&& apt-get upgrade \
&& sudo apt-get install pkg-config libssl-dev
RUN cargo build --release --bin cochinillos-voladores

FROM debian:bookworm-slim AS runtime
WORKDIR /cochinillos-voladores
COPY --from=builder /cochinillos-voladores/target/release/cochinillos-voladores /cochinillos-voladores
COPY --from=builder /cochinillos-voladores/assets /cochinillos-voladores/assets
ENTRYPOINT ["/cochinillos-voladores/cochinillos-voladores"]
EXPOSE 3000
