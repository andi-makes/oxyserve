FROM rust:slim AS build

WORKDIR /usr/src
RUN apt-get update && apt-get install -y \
  musl-tools \
  --no-install-recommends && \
  rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM scratch
WORKDIR /
COPY --from=build /usr/local/cargo/bin/oxyserve .
EXPOSE 8000
USER 1000
CMD ["./oxyserve"]