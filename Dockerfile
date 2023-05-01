# Step 1: build rust code
FROM rust:alpine as builder

RUN apk add --no-cache build-base musl-dev openssl-dev openssl

WORKDIR /code

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch
RUN cargo build --release
RUN rm src/main.rs

COPY src ./src/
# Update main file time to force rebuild
RUN touch src/main.rs
RUN cargo build --release

# Step 2: install Maelstrom
FROM amazoncorretto:20-al2-full

RUN yum update -y
RUN yum install -y graphviz gnuplot tar bzip2 git

WORKDIR /main

RUN curl -LJ https://github.com/jepsen-io/maelstrom/releases/download/v0.2.3/maelstrom.tar.bz2 | tar -xj
COPY --from=builder /code/target/release/ ./

ENTRYPOINT ["./maelstrom/maelstrom"]