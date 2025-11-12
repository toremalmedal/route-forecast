FROM rust:1.90 AS builder
WORKDIR /usr/src/myapp
RUN apt-get update && apt-get install protobuf-compiler -y
COPY . .
RUN cargo install --path . --features server

FROM debian:trixie-slim
RUN apt-get update && apt-get upgrade && ldconfig /usr/local/lib64
RUN apt-get install -y ca-certificates
RUN update-ca-certificates
COPY --from=builder /usr/local/cargo/bin/route-forecast-server /usr/local/bin/route-forecast-server
CMD ["route-forecast-server"]
