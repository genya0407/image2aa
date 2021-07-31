FROM rust:1.54.0-buster AS builder

WORKDIR /build
COPY . .
RUN cargo build --release --bin image2aa_web

FROM alpine:latest
WORKDIR /app
COPY web/* .
COPY --from=builder /build/target/release/image2aa_web /usr/bin/
CMD ["/usr/bin/image2aa_web"]
