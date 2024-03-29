FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /pin-bot

FROM chef AS planner
# prepare dependencies for caching
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
# build project dependencies
COPY --from=planner /pin-bot/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
# build project
COPY . .
RUN cargo build --release --bin pin-bot

FROM gcr.io/distroless/cc-debian12 as runtime
# copy missing dynamic libraries
COPY --from=chef --chown=root:root /lib/x86_64-linux-gnu/libz.so.1 /lib/x86_64-linux-gnu/libz.so.1

COPY --from=builder /pin-bot/target/release/pin-bot /pin-bot
ENTRYPOINT ["./pin-bot"]