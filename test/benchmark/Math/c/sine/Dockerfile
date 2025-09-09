FROM wasmedge/slim-runtime:0.10.1
WORKDIR /
ADD sine.wasm /
CMD ["wasmedge", "/sine.wasm", "40"]
