BINARY_NAME=sshs
INSTALL_DIR=$(HOME)/.local/bin

.PHONY: all build install clean
all: build install

build:
	@echo "Building..."
	cargo build --release

install:
	mkdir -p $(INSTALL_DIR)
	cp target/releases/$(BINARY_NAME) $(INSTALL_DIR)/$(BINARY_NAME)
	chmod +x $(INSTALL_DIR)/$(BINARY_NAME)
	@echo "Installed - ${INSTALL_DIR}/$(BINARY_NAME)"

clean:
	@echo "Cleaning dist..."
	rm -rf dist