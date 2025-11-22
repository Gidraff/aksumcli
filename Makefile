IP="192.168.4.0/24"

build:
	@echo "Building the project..."
	cargo build
run:
	@echo "Running the project..."
	./target/debug/aksumcli scan-ports -t ${IP}
fmt:
	@echo "Formatting the code..."
	cargo fmt
lint:
	@echo "Linting the code..."
	cargo clippy
test:
	@echo "Running tests..."
	cargo test

clean:
	@echo "Cleaning the project..."
	cargo clean
all: fmt lint build run
	@echo "All tasks completed.

# .Phony: build run fmt lint test clean all
.Phony: build run