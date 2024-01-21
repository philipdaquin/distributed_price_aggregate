FROM lukemathwalker/cargo-chef:latest-rust-1.75.0 as chef
WORKDIR /app
RUN apt-get update && \
  apt-get -y upgrade && \
  apt-get -y install pkg-config && \
  apt-get -y install libssl-dev  && \
  apt-get install -y libclang-dev && \
  apt-get install -y cmake  && \
  apt-get install -y build-essential && \
  apt-get clean
FROM chef as planner
COPY . .

# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json
FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

# ENV SQLX_OFFLINE true
# Build our project
RUN cargo build --release --bin worker_a
FROM debian:bookworm-slim AS runtime

WORKDIR /app
# COPY . . 
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates pkg-config libssl-dev libclang-dev cmake build-essential \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/worker_a worker_a
# COPY configuration configuration
# ENV APP_ENVIRONMENT production
EXPOSE 4001
ENTRYPOINT ["./worker_a"]