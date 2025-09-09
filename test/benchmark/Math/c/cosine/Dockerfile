FROM wasmedge/slim-runtime:0.10.1
WORKDIR /
ADD cosine.wasm /
CMD ["wasmedge", "/cosine.wasm", "40"]
