FROM rust:1.85

RUN apt-get update && apt-get install -y libpq-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

COPY . .

RUN cargo build

EXPOSE 8080

CMD ["cargo", "run"]
