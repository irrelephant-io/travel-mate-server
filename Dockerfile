# Nightly toolchains are required because of some async-fs deps
FROM rustlang/rust:nightly as builder

# Prebuild the dependencies in separate layer to take advantage of docker caching
# RUN USER=root cargo new travel-mate-server
WORKDIR /usr/src/travel-mate-server
#COPY Cargo.toml Cargo.lock ./
# RUN cargo build --release

# Build the actual app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY resource ./resource
COPY --from=builder /usr/local/cargo/bin/travel-mate-server /usr/local/bin/travel-mate-server
EXPOSE 8081
CMD [ "travel-mate-server" ]
