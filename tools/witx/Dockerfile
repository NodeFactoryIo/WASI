FROM rust:latest

WORKDIR /usr/src/witx

COPY . .
RUN cargo build --release
RUN cargo install --path .

ENTRYPOINT ["witx"]
