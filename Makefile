.PHONY: help check fmt clippy test build ci coverage bench
.PHONY: docker-up docker-down docker-logs db-reset
.PHONY: run clean

.DEFAULT_GOAL := help

help: ## Show this help message
	@echo "Dreadnought - Development Commands"
	@echo "===================================="
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'
	@echo ""
	@echo "Quick Start:"
	@echo "  make docker-up  # Start infrastructure"
	@echo "  make run        # Run application"
	@echo "  make test       # Run tests"
	@echo "  make ci         # Full CI pipeline"

check: ## Run cargo check
	@./scripts/check.sh

fmt: ## Check code formatting
	@./scripts/fmt.sh

fmt-fix: ## Fix code formatting
	@cargo fmt --all

clippy: ## Run clippy lints
	@./scripts/clippy.sh

test: ## Run all tests
	@./scripts/test.sh

build: ## Build the workspace
	@./scripts/build.sh

ci: ## Run full CI pipeline (fmt, clippy, test, build)
	@./scripts/ci.sh

coverage: ## Generate code coverage report
	@./scripts/coverage.sh

bench: ## Run benchmarks
	@./scripts/bench.sh

docker-up: ## Start Docker infrastructure
	@./scripts/docker-up.sh

docker-down: ## Stop Docker infrastructure
	@./scripts/docker-down.sh

docker-logs: ## View logs for a service (usage: make docker-logs SERVICE=postgres-timescale)
	@./scripts/docker-logs.sh $(SERVICE)

db-reset: ## Reset database (drops all data!)
	@./scripts/db-reset.sh

run: docker-up ## Run the application locally
	@cargo run --bin api_server

clean: ## Clean build artifacts
	@cargo clean
	@echo "==> Build artifacts cleaned"
