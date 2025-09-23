FROM rust:1.75-alpine AS crypto-builder
WORKDIR /app/crypto
COPY src/crypto/ .
RUN apk add --no-cache musl-dev openssl-dev pkgconfig
RUN cargo build --release

FROM golang:1.21-alpine AS controller-builder
WORKDIR /app/controller
COPY src/controllers/ .
RUN go mod download
RUN CGO_ENABLED=0 GOOS=linux go build -o qraiop-controller .

FROM python:3.11-slim AS final
WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy built artifacts
COPY --from=crypto-builder /app/crypto/target/release/libqraiop_crypto.so /usr/local/lib/
COPY --from=controller-builder /app/controller/qraiop-controller /usr/local/bin/

# Install Python dependencies
COPY src/agents/requirements.txt src/chaos/requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt

# Copy Python source code
COPY src/agents/ ./agents/
COPY src/chaos/ ./chaos/

# Create non-root user
RUN useradd -r -s /bin/false qraiop
USER qraiop

EXPOSE 8080 8443

CMD ["python", "-m", "agents.supervisor"]
