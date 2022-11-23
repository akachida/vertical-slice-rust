FROM rust:1.64.0 as builder

WORKDIR /usr/src/backend-api
COPY . .
RUN cargo install --path .

FROM debian:buster-slim

RUN apt-get update && \
    apt-get install -y libssl-dev && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/backend-api /usr/local/bin/backend-api

ENV DATABASE_URL_READ=
ENV DATABASE_URL_WRITE=
ENV DATABASE_URL_TEST=
ENV TEST_DATABASE_NAME=
ENV AUTH_SECRET=
ENV REFRESH_TOKEN_SECRET=
ENV PASSWORD_SECRET=
ENV RUST_LOG=warn

EXPOSE 8080
EXPOSE 5432

CMD ["backend-api"]
