FROM rust:slim as builder

# muslc is required in order to build the rust image.
RUN apt-get update && \
 apt-get -y install ca-certificates cmake musl-tools libssl-dev && \
 rm -rf /var/lib/apt/lists/* && \
 rustup target add x86_64-unknown-linux-musl
# Sets the environment variable for the cargo build command that follows.
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /app

COPY . .

RUN cargo build --target x86_64-unknown-linux-musl --release

################### Stage 2 #################
FROM alpine

RUN apk update && apk add bash

WORKDIR /app

VOLUME /app/config

ENV PORT 8080

EXPOSE $PORT

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/whl ./whl
COPY run.sh .

ENTRYPOINT ["./run.sh"]