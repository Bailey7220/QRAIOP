```makefile
.PHONY: help build test clean install security-scan lint format
.DEFAULT_GOAL := help

# Variables
RUST_DIR := src/crypto
PYTHON_DIRS := src/agents src/chaos tests
GO_DIR := src/controllers
DOCKER_REGISTRY := ghcr.io/bailey7220
IMAGE_TAG := $(shell git rev-parse --short HEAD)

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-15s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

install: ## Install all dependencies
	@echo "Installing Rust dependencies..."
	cd $(RUST_DIR) && cargo build --release
	@echo "Installing Python dependencies..."
	pip install -r src/agents/requirements.txt
	pip install -r src/chaos/requirements.txt
	@echo "Installing Go dependencies..."
	cd $(GO_DIR) && go mod tidy

build: ## Build all components
	@echo "Building Rust crypto library..."
	cd $(RUST_DIR) && cargo build --release
	@echo "Building Go controllers..."
	cd $(GO_DIR) && go build -o bin/qraiop-controller ./...
	@echo "Building Docker images..."
	docker build -t $(DOCKER_REGISTRY)/qraiop:$(IMAGE_TAG) .

test: ## Run all tests
	@echo "Running Rust tests..."
	cd $(RUST_DIR) && cargo test --verbose
	@echo "Running Python tests..."
	python -m pytest tests/ -v --cov=src/
	@echo "Running Go tests..."
	cd $(GO_DIR) && go test ./...

security-scan: ## Run security scans
	@echo "Running Rust security audit..."
	cd $(RUST_DIR) && cargo audit
	@echo "Running Python security scan..."
	safety check -r src/agents/requirements.txt
	safety check -r src/chaos/requirements.txt
	@echo "Running Go security scan..."
	cd $(GO_DIR) && gosec ./...
	@echo "Running container security scan..."
	trivy image $(DOCKER_REGISTRY)/qraiop:$(IMAGE_TAG)

lint: ## Run linters
	@echo "Linting Rust code..."
	cd $(RUST_DIR) && cargo clippy -- -D warnings
	@echo "Linting Python code..."
	flake8 $(PYTHON_DIRS)
	black --check $(PYTHON_DIRS)
	@echo "Linting Go code..."
	cd $(GO_DIR) && golangci-lint run

format: ## Format code
	@echo "Formatting Rust code..."
	cd $(RUST_DIR) && cargo fmt
	@echo "Formatting Python code..."
	black $(PYTHON_DIRS)
	@echo "Formatting Go code..."
	cd $(GO_DIR) && go fmt ./...

clean: ## Clean build artifacts
	cd $(RUST_DIR) && cargo clean
	cd $(GO_DIR) && rm -rf bin/
	docker system prune -f

deploy: ## Deploy to Kubernetes
	kubectl apply -f configs/k8s/
	kubectl rollout status deployment/qraiop-controller

benchmark: ## Run performance benchmarks
	cd $(RUST_DIR) && cargo bench
	python -m pytest tests/performance/ -v

docs: ## Generate documentation
	cd $(RUST_DIR) && cargo doc --no-deps
	mkdocs build
