FROM rust:latest

WORKDIR /usr/src/edgecase

COPY . .


RUN cargo build --release

ENV HOST=0.0.0.0
ENV PORT=3000

EXPOSE 3000


CMD ["./target/release/EdgeCase"]