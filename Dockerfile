# Load rust image 
FROM rust:latest as builder

# Create new project
RUN USER=root cargo new --bin looker
WORKDIR /looker


# Copy the files
COPY ./looker/Cargo.toml ./Cargo.toml
COPY ./looker/src ./src

# Install cmake
RUN apt-get update
RUN apt-get install -y cmake

# Build the app with the release flag
RUN cargo build --release

# Create a lighter image using debian
FROM ubuntu:latest

RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y openssl
RUN apt-get install -y build-essential

# Copy the bin
COPY --from=builder /looker/target/release/looker looker

ENV RUST_LOG=looker

# Run looker
ENTRYPOINT [ "./looker" ]

