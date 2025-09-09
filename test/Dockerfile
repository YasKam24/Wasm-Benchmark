FROM alpine:3.18 AS builder
RUN apk add --no-cache curl tar python3

# Install WasmEdge runtime
RUN curl -sSfL https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | sh

FROM alpine:3.18 AS runtime
COPY --from=builder /usr/local/bin/wasmedge /usr/local/bin/wasmedge
# Add only necessary shared libs (if any) or use static build

WORKDIR /app
ENTRYPOINT ["wasmedge"]

# Then later override with your wasm:
# CMD ["--dir", ".:/", "/app/your_app.wasm"]
