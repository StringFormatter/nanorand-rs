# Build Stage
FROM ubuntu:20.04 as builder

## Install build dependencies.
RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y build-essential cargo

## Add source code to the build stage.
COPY . /

## Build Mayhem tests
WORKDIR /nanorand-mayhem
RUN cargo build
RUN cargo build --release

## Build libfuzzer tests
WORKDIR ../nanorand-fuzz
RUN cargo build
RUN cargo build --release

# Package Stage
FROM ubuntu:20.04

## TODO: Change <Path in Builder Stage>
COPY --from=builder /nanorand-mayhem/target/debug/chacha20 /chacha20-m-dev
COPY --from=builder /nanorand-mayhem/target/debug/range /range-m-dev
COPY --from=builder /nanorand-mayhem/target/release/chacha20 /chacha20-m
COPY --from=builder /nanorand-mayhem/target/release/range /range-m
COPY --from=builder /nanorand-fuzz/target/debug/chacha20 /chacha20-dev
COPY --from=builder /nanorand-fuzz/target/debug/range /range-dev
COPY --from=builder /nanorand-fuzz/target/release/chacha20 /chacha20
COPY --from=builder /nanorand-fuzz/target/release/range /range
