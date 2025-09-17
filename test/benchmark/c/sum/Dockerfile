FROM wasmedge/slim-runtime:0.10.1
WORKDIR /
ADD sum.wasm /
CMD ["wasmedge", "/sum.wasm", "10000"]
