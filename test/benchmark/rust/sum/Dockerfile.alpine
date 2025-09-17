FROM wasmedge/slim-runtime:0.10.1
WORKDIR /
ADD /target/wasm32-wasip1/release/sum.wasm /
CMD ["wasmedge", "/sum.wasm", "10000"]
