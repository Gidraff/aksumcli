build:
	@echo "Building the project..."
	cargo build
run:
	@echo "Running the project..."
	cargo run 
fmt:
	@echo "Formatting the code..."
	cargo fmt
lint:
	@echo "Linting the code..."
	cargo clippy
test:
	@echo "Running tests..."
	cargo test