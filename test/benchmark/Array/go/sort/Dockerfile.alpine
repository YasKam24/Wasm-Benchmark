FROM wasmedge/slim-runtime:0.10.1
WORKDIR /
ADD sort.wasm /
CMD ["wasmedge", "/sort.wasm", "10000"]
