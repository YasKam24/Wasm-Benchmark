FROM wasmedge/slim-runtime:0.10.1
WORKDIR /
ADD /target/wasm32-wasip1/release/search.wasm /
CMD ["wasmedge", "/search.wasm", "10000"]
