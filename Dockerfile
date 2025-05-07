
FROM rust:1.76 as builder


WORKDIR /usr/src/edgecase


COPY Cargo.toml Cargo.lock* ./


RUN mkdir -p src && \
    echo "fn main() {println!(\"dummy\")}" > src/main.rs && \
    cargo build --release && \
    rm -r src


COPY . .
RUN cargo build --release


FROM debian:bookworm-slim


RUN apt-get update && \
    apt-get install -y ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*


COPY --from=builder /usr/src/edgecase/target/release/EdgeCase /usr/local/bin/edgecase


COPY --from=builder /usr/src/edgecase/jokes.json /usr/local/bin/jokes.json


WORKDIR /usr/local/bin


ENV HOST=0.0.0.0
ENV PORT=3000


EXPOSE 3000


CMD ["edgecase"]