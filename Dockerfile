FROM rust:1.85
COPY ./ ./

RUN cargo build --release

EXPOSE 3000

CMD ["./target/release/rocket-backend-template"] 