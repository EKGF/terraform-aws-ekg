ifndef GIT_ROOT
GIT_ROOT := $(shell git rev-parse --show-toplevel 2>/dev/null)
endif

MK_DIR := $(GIT_ROOT)/.make

-include $(GIT_ROOT)/ekgf-make.mk

.PHONY: build
build: cargo-check cargo-lambda-check
	$(CARGO_BIN) lambda build

.PHONY: watch
watch: cargo-check cargo-lambda-check
	$(CARGO_BIN) lambda watch --wait -vv

.PHONY: invoke
invoke: cargo-check cargo-lambda-check event.json
	$(CARGO_BIN) lambda invoke --data-file event.json -vv