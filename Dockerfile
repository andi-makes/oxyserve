FROM rust:1.52.1 AS build

WORKDIR /usr/src

RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new app
WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
RUN cargo install --target x86_64-unknown-linux-musl --path .

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
WORKDIR /
COPY --from=build /usr/local/cargo/bin/andi-makes-dev .
COPY static ./static/
COPY templates ./templates/
COPY Rocket.toml ./
EXPOSE 8000
USER 1000
CMD ["./andi-makes-dev"]