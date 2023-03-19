####################################################################################################
## Build image
####################################################################################################
FROM rust:alpine as builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apk add --no-cache musl-dev

COPY . .

RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM alpine:latest

COPY --from=builder /target/x86_64-unknown-linux-musl/release/chatr ./bin
RUN ls -a

ENTRYPOINT ["chatr"]