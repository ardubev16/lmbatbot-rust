FROM rust AS builder

# create a new empty shell project
RUN USER=root cargo new --bin lmbatbot-rust
WORKDIR /lmbatbot-rust

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/lmbatbot_rust*
RUN cargo build --release

FROM ubuntu
WORKDIR /app
RUN apt-get update && apt-get install -y wget && rm -rf /var/lib/apt/lists/*
RUN wget http://nz2.archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.1f-1ubuntu2.16_amd64.deb
RUN dpkg -i libssl1.1_1.1.1f-1ubuntu2.16_amd64.deb
COPY --from=builder /lmbatbot-rust/target/release/lmbatbot-rust .

CMD ["./lmbatbot-rust"]
