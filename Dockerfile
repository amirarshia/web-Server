FROM rust:1

WORKDIR /server

COPY Cargo.* .

RUN cargo fetch

COPY . .

EXPOSE 8080

RUN cargo build --release

CMD ./target/release/webserver