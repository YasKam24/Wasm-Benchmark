FROM wasmedge/slim-runtime:0.10.1
WORKDIR /
ADD /target/wasm32-wasip1/release/reverse.wasm /
CMD ["wasmedge", "/reverse.wasm", "10000"]
