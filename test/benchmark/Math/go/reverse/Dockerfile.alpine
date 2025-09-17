FROM wasmedge/slim-runtime:0.10.1
WORKDIR /
ADD reverse.wasm /
CMD ["wasmedge", "/reverse.wasm", "10000"]
