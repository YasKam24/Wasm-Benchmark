FROM wasmedge/slim-runtime:0.10.1
WORKDIR /
ADD /target/wasm32-wasip1/release/factorial.wasm /
CMD ["wasmedge", "/factorial.wasm", "40"]
