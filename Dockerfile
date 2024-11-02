FROM rust AS builder

WORKDIR /app

COPY . .

RUN cargo install --path .

FROM debian
RUN apt-get update && apt-get install -y libc6 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/shortener /usr/local/bin/shortener

CMD ["shortener"]