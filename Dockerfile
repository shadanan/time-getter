FROM rust AS rust

WORKDIR /app

COPY backend/Cargo.toml backend/Cargo.lock ./
COPY backend/src ./src

RUN cargo build --release

FROM node AS node

WORKDIR /app

COPY frontend/*.json frontend/*.js frontend/*.ts ./
RUN npm install

COPY frontend/src ./src
COPY frontend/static ./static
RUN npm run build

FROM gcr.io/distroless/cc-debian12

WORKDIR /app

COPY --from=rust /app/target/release/timegetter ./
COPY --from=node /app/build ./static

ENV PORT=8000
ENV ROCKET_ADDRESS=0.0.0.0

CMD ["./timegetter"]
