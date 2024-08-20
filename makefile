PROJECT_NAME := $(shell grep '^name' Cargo.toml | sed -e 's/.*= "\(.*\)"/\1/')

# Distination install dir
INSTALL_DIR := /usr/local/bin

# ビルドされたバイナリのパス
BINARY_PATH := target/release/$(PROJECT_NAME)

# バイナリの名前
BINARY_NAME := "ulid"

.PHONY: all build install uninstall clean

all: build install

build:
	cargo build --release

install: build
	@echo "Installing $(PROJECT_NAME) to $(INSTALL_DIR)"
	@sudo install -m 755 $(BINARY_PATH) $(INSTALL_DIR)/$(PROJECT_NAME)
	@sudo ln -s $(INSTALL_DIR)/$(PROJECT_NAME) $(INSTALL_DIR)/$(BINARY_NAME)

uninstall:
	@echo "Uninstalling $(PROJECT_NAME) from $(INSTALL_DIR)/ulid"
	@sudo rm -f $(INSTALL_DIR)/$(BINARY_NAME)

clean:
	cargo clean
	@echo "Cleaned build artifacts"