# Build Stage
FROM ubuntu:20.04 as builder

## Install build dependencies.
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y cmake cargo

## Add source code to the build stage.
ADD . /
WORKDIR /nanorand-rs

## Build Mayhem tests
WORKDIR /nanorand-mayhem
RUN Cargo build
RUN Cargo build -r

## Build libfuzzer tests
WORKDIR ../nanorand-fuzz
RUN Cargo build
RUN Cargo build -r

# Package Stage
FROM ubuntu:20.04

## TODO: Change <Path in Builder Stage>
COPY --from=builder /nanorand-rs/nanorand-mayhem/target/debug/chacha20 /chacha20-m-dev
COPY --from=builder /nanorand-rs/nanorand-mayhem/target/debug/range /range-m-dev
COPY --from=builder /nanorand-rs/nanorand-mayhem/target/release/chacha20 /chacha20-m
COPY --from=builder /nanorand-rs/nanorand-mayhem/target/release/range /range-m
COPY --from=builder /nanorand-rs/nanorand-fuzz/target/debug/chacha20 /chacha20-dev
COPY --from=builder /nanorand-rs/nanorand-fuzz/target/debug/range /range-dev
COPY --from=builder /nanorand-rs/nanorand-fuzz/target/release/chacha20 /chacha20
COPY --from=builder /nanorand-rs/nanorand-fuzz/target/release/range /range
