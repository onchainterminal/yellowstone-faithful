DEFAULT := plugin

# Path: Makefile
all: plugin faithful

plugin:
	@echo "Building plugin..."
	@cd demo-plugin && cargo build --release --lib --target-dir=target

faithful:
	@echo "Building rust faithful..."
	@cargo build --release --target-dir=target
