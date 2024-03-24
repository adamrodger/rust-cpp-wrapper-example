# Builder counter-cpp
FROM ubuntu AS counter-cpp
RUN apt-get update \
 && apt-get install -y clang

WORKDIR /src
COPY counter-cpp/* .
RUN mkdir -p artifacts \
 && c++ -shared -o artifacts/libcounter.so Counter.cc

# Build and test counter-rs
FROM rust:1.77.0 AS counter-rs
RUN apt-get update \
 && apt-get install -y clang libclang-dev

WORKDIR /src
COPY --from=counter-cpp /src/artifacts/libcounter.so /usr/local/lib/libcounter.so
COPY . .
RUN cargo build

ENV LD_LIBRARY_PATH=/usr/local/lib
RUN cargo test