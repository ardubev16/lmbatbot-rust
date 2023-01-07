FROM rust as builder

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

FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /lmbatbot-rust/target/release/lmbatbot-rust .

CMD ["./lmbatbot-rust"]
