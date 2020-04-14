FROM rust AS wasm-build

RUN rustup target install wasm32-unknown-unknown && cargo install wasm-bindgen-cli
WORKDIR /usr/src/app
COPY Cargo.lock .
COPY Cargo.toml .
RUN mkdir .cargo && mkdir src && touch src/lib.rs
RUN cargo build --target wasm32-unknown-unknown --release

COPY ./src src
RUN touch src/lib.rs

RUN cargo build --target wasm32-unknown-unknown --release
RUN wasm-bindgen --target web --no-typescript --out-dir static target/wasm32-unknown-unknown/release/singular.wasm

FROM rust as lobbier-build

RUN git clone https://github.com/etwyniel/lobbier
RUN cd lobbier && cargo install --path .
ARG CACHE_DATE
RUN echo $CACHE_DATE && cd lobbier && git pull && cargo install --path . --force

FROM debian:stable-slim

COPY --from=lobbier-build /usr/local/cargo/bin/lobbier /bin
COPY ./static static

CMD ["lobbier"]
