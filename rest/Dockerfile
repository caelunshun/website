FROM rust:latest

WORKDIR /usr/src/feather-rest
COPY . .

RUN cargo install --path .

EXPOSE 3030

CMD ["feather-rest"]