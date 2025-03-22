FROM rust:1.85-alpine AS builder
WORKDIR /app
COPY . .
ENV OPENSSL_LIB_DIR=/usr/lib
ENV OPENSSL_INCLUDE_DIR=/usr/include

RUN apk add --no-cache build-base musl-dev libpq-dev openssl-dev gcc    && \
    cargo build --release                                               && \
    cargo build --release -p migration

FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/target/release/orm_example /usr/local/bin/orm_example
COPY --from=builder /app/target/release/migration /usr/local/bin/migration
RUN apk add --no-cache libpq
    
CMD ["sh", "-c", "migration && orm_example"]


# CMD ["sleep", "infinity"]
