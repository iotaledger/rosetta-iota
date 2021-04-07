############################
# Prepare dependencies recipe
############################
FROM lukemathwalker/cargo-chef as planner

WORKDIR /rosetta-iota
COPY . .

RUN cargo chef prepare --recipe-path recipe.json

############################
# Dependency cache
############################
FROM lukemathwalker/cargo-chef as cacher

RUN apt-get update && \
    apt-get install cmake clang -y

WORKDIR /rosetta-iota
COPY --from=planner /rosetta-iota/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

############################
# Build
############################
FROM rust:1 as build

WORKDIR /rosetta-iota
COPY . .

COPY --from=cacher /rosetta-iota/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME

RUN cargo build --release

############################
# Image
############################
FROM ubuntu:focal

RUN apt-get update && \
    apt-get install openssl -y

RUN rm -rf /var/lib/apt

# API
EXPOSE 3030/tcp

COPY --from=build /rosetta-iota/target/release/rosetta-iota /
COPY --from=build /rosetta-iota/target/release/rosetta-iota-utils /

ENTRYPOINT ["/rosetta-iota"]
