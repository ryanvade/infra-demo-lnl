FROM node:14 AS NODE-CACHER
WORKDIR /usr/src/todos
COPY ["./client/package.json", "./client/yarn.lock", "./"]
RUN yarn install

FROM node:14 AS NODE-BUILDER
WORKDIR /usr/src/todos
COPY "./client" "./"
COPY --from=NODE-CACHER /usr/src/todos/node_modules ./node_modules
RUN yarn build

FROM rust:1.51 AS RUST-PLANNER
WORKDIR /usr/src/todos-api
RUN cargo install cargo-chef
COPY "./api" "./"
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:1.51 AS RUST-CACHER
WORKDIR /usr/src/todos-api
RUN cargo install cargo-chef
COPY --from=RUST-PLANNER /usr/src/todos-api/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.51 AS RUST-BUILDER
WORKDIR /usr/src/todos-api
COPY "./api" "./"
COPY --from=RUST-CACHER /usr/src/todos-api/target target
COPY --from=RUST-CACHER /usr/local/cargo /usr/local/cargo

RUN cargo install --path .

FROM rust:1.51-slim
WORKDIR /opt/todos-api
COPY --from=RUST-BUILDER /usr/local/cargo/bin/todos-api .
COPY --from=NODE-BUILDER /usr/src/todos/dist ./dist

CMD ["./todos-api"]
