FROM rust:1.54.0-buster AS builder

WORKDIR /build
RUN mkdir core
RUN mkdir web
COPY Cargo.toml .
COPY Cargo.lock .
COPY core/Cargo.toml core
COPY web/Cargo.toml web
RUN mkdir web/src && echo 'fn main() { println!("Hello, world!"); }' > web/src/main.rs
RUN cargo build --release --bin image2aa_web
RUN rm web/src/main.rs
COPY . .
RUN cargo build --release --bin image2aa_web

FROM debian:buster
WORKDIR /app
COPY web/ .
COPY --from=builder /build/target/release/image2aa_web /usr/bin/
CMD ["/usr/bin/image2aa_web"]
