FROM wasmedge/slim-runtime:0.10.1
WORKDIR /
ADD /target/wasm32-wasip1/release/sort.wasm /
CMD ["wasmedge", "/sort.wasm", "10000"]
