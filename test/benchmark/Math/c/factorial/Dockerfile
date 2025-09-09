FROM wasmedge/slim-runtime:0.10.1
WORKDIR /
ADD factorial.wasm /
CMD ["wasmedge", "/factorial.wasm", "40"]
