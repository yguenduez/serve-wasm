#!/bin/bash

# Generate a self-signed TLS certificate for development
# Usage: ./gen-cert.sh

set -e

CERT_FILE="cert.pem"
KEY_FILE="key.pem"
DAYS=365

echo "Generating self-signed TLS certificate..."

# Generate a private key and certificate
openssl req -x509 -newkey rsa:4096 -keyout "$KEY_FILE" -out "$CERT_FILE" -days "$DAYS" -nodes \
    -subj "/C=US/ST=State/L=City/O=Organization/CN=localhost"

echo "  Certificate generated successfully!"
echo "  Certificate: $CERT_FILE"
echo "  Private Key: $KEY_FILE"
