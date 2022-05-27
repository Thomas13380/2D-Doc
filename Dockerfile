FROM rust:1.59.0 as builder
RUN mkdir /usr/src/2ddoc
WORKDIR /usr/src/2ddoc
COPY . .
RUN rustup default 1.59.0
RUN cargo build --release
EXPOSE 4000

FROM gcr.io/distroless/cc-debian11

COPY --from=builder /usr/src/2ddoc/target/release/smart2ddoc /usr/src/2ddoc


CMD ["./smart2ddoc"]





