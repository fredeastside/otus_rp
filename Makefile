.PHONY: run-cli
run-cli:
	@echo "Running CLI..."
	@cargo run -p sh-cli

.PHONY: test
test:
	@echo "Running tests..."
	@cargo test

.PHONY: lint
lint:
	@echo "Running linters..."
	@cargo clippy --all-targets --all-features -- -D warnings

.PHONY: fmt
fmt:
	@echo "Running fmt..."
	@cargo fmt --check
