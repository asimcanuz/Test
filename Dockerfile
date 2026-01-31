# --- STAGE 1: BUILDER ---
FROM rust:1.84 as builder

WORKDIR /usr/src/app
COPY . .

# Release modunda derle (Compile in release mode)
RUN cargo build --release

# --- STAGE 2: RUNTIME ---
FROM debian:bookworm-slim

# HTTPS ve sistem gereksinimleri (System requirements)
RUN apt-get update && apt-get install -y \
    ca-certificates \
        libssl-dev \
            && rm -rf /var/lib/apt/lists/*

            WORKDIR /app

            # Binary dosyasını kopyala (Copy binary)
            COPY --from=builder /usr/src/app/target/release/cdn-service /app/cdn-service

            # Uploads klasörünü oluştur (Create uploads folder)
            RUN mkdir -p /app/uploads

            EXPOSE 8080

            CMD ["./cdn-service"]