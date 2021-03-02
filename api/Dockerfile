FROM lukemathwalker/cargo-chef as planner
WORKDIR app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM lukemathwalker/cargo-chef as cacher
WORKDIR app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust as builder
WORKDIR app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
ENV SQLX_OFFLINE true
RUN cargo build --release

# TODO alpine with required packages
FROM rust as runtime
WORKDIR app
COPY --from=builder /app/target/release/feather-web-api /usr/local/bin
COPY --from=builder /app/migrations
EXPOSE 6000
ENTRYPOINT ["/usr/local/bin/feather-web-api"]