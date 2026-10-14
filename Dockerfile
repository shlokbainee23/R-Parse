# ==============================================================================
# R-PARSE — Reproducible Multi-Stage Production Dockerfile
#
# Builds the deterministic Rust analysis core and bundles the unified website
# (Landing Page, Scientific Analysis Studio, and live API backend).
# ==============================================================================

# Stage 1: Build Rust release binary with tree-sitter C dependencies
FROM rust:1.80-slim-bookworm AS builder

WORKDIR /usr/src/rparse

# Install C compiler required for tree-sitter parser build.rs
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Copy repository sources (preserving workspace relative paths for include_str!)
COPY implementation ./implementation
COPY landing.html ./landing.html
COPY how-to-use.html ./how-to-use.html
COPY index.html ./index.html
COPY landing ./landing
COPY favicon.svg ./favicon.svg

# Compile the authoritative CLI binary in release mode
WORKDIR /usr/src/rparse/implementation
RUN cargo build --release --package rparse-cli

# Stage 2: Minimal runtime container
FROM debian:bookworm-slim AS runner

# Install TLS certificates and curl for container health check probes
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
 && rm -rf /var/lib/apt/lists/*

# Create unprivileged application user
RUN useradd -m -u 1000 -s /bin/bash rparse

WORKDIR /home/rparse/app

# Copy compiled self-contained binary from builder
COPY --from=builder /usr/src/rparse/implementation/target/release/rparse /usr/local/bin/rparse

# Copy showcase fixtures for analysis exploration
COPY --chown=rparse:rparse implementation/fixtures ./fixtures

# Production networking environment defaults
ENV PORT=8080
ENV HOST=0.0.0.0
ENV RUST_LOG=info

EXPOSE 8080

# Health check probing same-origin health endpoint
HEALTHCHECK --interval=30s --timeout=5s --start-period=5s --retries=3 \
    CMD curl -f "http://localhost:${PORT}/api/health" || exit 1

# Run unprivileged
USER rparse

# Start the unified web service and analysis studio automatically
CMD ["rparse", "studio"]
