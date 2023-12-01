FROM rust:1.74 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM ubuntu:22.04 
COPY --from=builder /usr/local/cargo/bin/totoro /usr/local/bin/totoro
CMD ["totoro"]