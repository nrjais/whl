FROM alpine

RUN apk update && apk add bash

WORKDIR /app

VOLUME /app/config

ENV PORT 8080

EXPOSE $PORT

COPY target/x86_64-unknown-linux-musl/release/whl ./whl
COPY run.sh .

ENTRYPOINT ["./run.sh"]