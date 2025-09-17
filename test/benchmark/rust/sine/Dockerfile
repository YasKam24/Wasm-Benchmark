FROM wasmedge/slim-runtime:0.10.1
WORKDIR /
ADD /target/wasm32-wasip1/release/sine.wasm /
CMD ["wasmedge", "/sine.wasm", "40"]
