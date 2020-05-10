FROM rustlang/rust:nightly as builder

RUN apt-get update\
    && yes yes | apt-get install cmake \
    && yes yes | apt-get install protobuf-compiler

RUN USER=root cargo new --bin side-boat-builder 

WORKDIR /side-boat-builder/

COPY Cargo.lock ./Cargo.lock
COPY Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src
COPY build.rs .

# build for release
RUN rm ./target/release/deps/side_boat*
RUN cargo build --release

# Strip debug symbols out of binary
RUN strip /side-boat-builder/target/release/side-boat

FROM debian:stable-slim

RUN apt-get update\
	&& yes yes | apt-get install openssl
WORKDIR /side-boat

COPY --from=builder /side-boat-builder/target/release/side-boat /side-boat/side-boat

CMD /side-boat/side-boat
