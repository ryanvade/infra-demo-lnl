FROM node:14 AS NODE-BUILDER
WORKDIR /usr/src/todos

COPY ["./client/package.json", "./client/yarn.lock", "./"]

RUN yarn install

COPY "./client" "./"

RUN yarn build

FROM rust:1.51 AS RUST-BUILDER
WORKDIR /usr/src/todos-api
COPY "./api" "./"

RUN cargo build --release
RUN cargo install --path .

FROM rust:1.51
WORKDIR /opt/todos-api
COPY --from=RUST-BUILDER /usr/local/cargo/bin/todos-api .
COPY --from=NODE-BUILDER /usr/src/todos/dist ./dist

RUN ls .

CMD ["./todos-api"]
