FROM ubuntu:latest

COPY target/dx/tobydavis-dev/release/web/server /app/server
COPY target/dx/tobydavis-dev/release/web/public /app/public

WORKDIR /app

CMD ["./server"]
