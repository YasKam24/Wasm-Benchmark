FROM wasmedge/slim-runtime:0.10.1
WORKDIR /
ADD /target/wasm32-wasip1/release/cosine.wasm /
CMD ["wasmedge", "/cosine.wasm", "40"]
