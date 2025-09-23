#!/bin/bash
# scripts/setup.sh

set -e

echo "🚀 Setting up QRAIOP development environment..."

# Check prerequisites
check_command() {
    if ! command -v $1 &> /dev/null; then
        echo "❌ $1 is not installed. Please install it first."
        exit 1
    else
        echo "✅ $1 found"
    fi
}

echo "📋 Checking prerequisites..."
check_command "rust"
check_command "python3"
check_command "go"
check_command "docker"
check_command "kubectl"

# Install Rust dependencies
echo "🦀 Installing Rust dependencies..."
cd src/crypto
cargo build --release
cd ../..

# Install Python dependencies
echo "🐍 Installing Python dependencies..."
python3 -m venv venv
source venv/bin/activate
pip install -r src/agents/requirements.txt
pip install -r src/chaos/requirements.txt
pip install -r tests/requirements.txt

# Install Go dependencies
echo "🐹 Installing Go dependencies..."
cd src/controllers
go mod tidy
cd ../..

# Create necessary directories
echo "📁 Creating directories..."
mkdir -p {logs,data,certificates}

# Generate development certificates
echo "🔐 Generating development certificates..."
./scripts/generate-certs.sh

# Build all components
echo "🏗️ Building components..."
make build

echo "✅ Setup complete! Run 'docker-compose up' to start the development environment."
