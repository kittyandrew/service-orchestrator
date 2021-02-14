FROM rustlang/rust:nightly-slim as builder
WORKDIR /usr/src/app
# Install dependencies
RUN apt-get update \
 && apt-get dist-upgrade -y \
 && apt-get install -qq -y \
    musl-tools \
    libssl-dev \
    openssl \
    pkg-config \
 && rustup target add x86_64-unknown-linux-musl
# Create project
RUN USER=root cargo new app
# Copying config/build files
COPY src src
COPY Cargo.toml .
COPY Cargo.lock .
# RUN cargo install --locked --target x86_64-unknown-linux-musl --path .
RUN cargo install --locked --target x86_64-unknown-linux-gnu --path .


FROM scratch
COPY --from=builder /usr/local/cargo/bin/service-orchestrator .
# Copying config file for prod
COPY Rocket.toml .
USER 1000
CMD ["./service-orchestrator"]
