FROM rust AS wasm

RUN rustup target install wasm32-unknown-unknown

COPY . .

RUN cargo build --target wasm32-unknown-unknown --release

FROM rust

RUN cargo install wasm-bindgen-cli
RUN git clone https://github.com/etwyniel/lobbier
RUN cd lobbier && cargo install --path .
COPY --from=wasm static static
COPY --from=wasm target/wasm32-unknown-unknown/release/singular.wasm .
RUN wasm-bindgen --target web --no-typescript --out-dir static singular.wasm

CMD ["lobbier"]
