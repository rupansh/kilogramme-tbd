FROM lukemathwalker/cargo-chef:latest-rust-1.53.0-slim-buster as planner
WORKDIR app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM lukemathwalker/cargo-chef as cacher
WORKDIR app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.53.0-slim-buster as build

WORKDIR /usr/src/app

COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
COPY . .

RUN cargo build --release

FROM debian:buster-slim as runner

WORKDIR /usr/app

COPY --from=build /usr/src/app/target/release/kilogramme-tbd .
COPY ./config.toml .

CMD ["./kilogramme-tbd"]
