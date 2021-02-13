FROM rustlang/rust:nightly-slim as builder
WORKDIR /usr/src/app
# Install dependencies
RUN apt-get update \
 && apt-get dist-upgrade -y \
 && apt-get install -y musl-tools \
 && rustup target add x86_64-unknown-linux-musl
# Create project
RUN USER=root cargo new orchestrator
# Copying config/build files
COPY src src
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo install --locked --target x86_64-unknown-linux-musl --path .


FROM scratch
COPY --from=builder /usr/local/cargo/bin/orchestrator .
# Copying config file for prod
COPY Rocket.toml .
USER 1000
CMD ["./orchestrator"]
