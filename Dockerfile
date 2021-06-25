FROM rust:1.52.1 AS build

WORKDIR /usr/src

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
WORKDIR /
COPY --from=build /usr/local/cargo/bin/oxyserve .
COPY Rocket.toml ./
EXPOSE 8000
USER 1000
CMD ["./oxyserve"]