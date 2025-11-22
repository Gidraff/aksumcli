NAME="aksumcli"

build:
	@echo "Building the project..."
	cargo build
run:
	@echo "Running the project..."
	./target/debug/aksumcli --name ${NAME}
fmt:
	@echo "Formatting the code..."
	cargo fmt
lint:
	@echo "Linting the code..."
	cargo clippy
test:
	@echo "Running tests..."
	cargo test