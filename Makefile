
Makefile:: ;

include ekgf-make.mk

.PHONY: all
all:
	@echo "all"

.PHONY: build-lambda-invoke
build-lambda-invoke: poetry-check
	@echo "build-lambda-invoke"
	cd $(GIT_ROOT)/lambda/invoke && $(POETRY_BIN) build

.PHONY: build-lambda-load
build-lambda-load: poetry-check
	@echo "build-lambda-load"
	cd $(GIT_ROOT)/lambda/load && $(POETRY_BIN) build

.PHONY: build-lambda-check
build-lambda-check: poetry-check
	@echo "build-lambda-check"
	cd $(GIT_ROOT)/lambda/check && $(POETRY_BIN) build

.PHONY: build
build: build-lambda-invoke build-lambda-load build-lambda-check