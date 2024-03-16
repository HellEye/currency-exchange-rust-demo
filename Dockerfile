FROM rust:alpine
WORKDIR /app
RUN apk add musl-dev libressl-dev --update && \
  rm -rf /var/cache/apk/*
COPY .env .
COPY Cargo.* .
COPY src ./src
ENTRYPOINT [ "cargo", "test" ]