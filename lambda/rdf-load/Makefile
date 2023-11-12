ifdef GIT_ROOT
#$(info GIT_ROOT is $(GIT_ROOT))
else
GIT_ROOT := $(shell git rev-parse --show-toplevel 2>/dev/null)
endif

MK_DIR := $(GIT_ROOT)/.make

include $(MK_DIR)/os.mk
include $(MK_DIR)/make.mk
include $(MK_DIR)/sops.mk
include $(MK_DIR)/git.mk
include $(MK_DIR)/zip.mk
include $(MK_DIR)/python.mk
include $(MK_DIR)/pip.mk
include $(MK_DIR)/poetry.mk

LAMBDA_FUNCTION_NAME := rdf-load
LAMBDA_FUNCTION_DIR := $(GIT_ROOT)/infra/aws/modules/rdf-load/lambda/$(LAMBDA_FUNCTION_NAME)
LAMBDA_FUNCTION_PACKAGE_DIR := $(LAMBDA_FUNCTION_DIR)/.package
LAMBDA_FUNCTION_ARTIFACT_ZIP := $(LAMBDA_FUNCTION_DIR)/artifact.zip
LAMBDA_FUNCTION_SOURCE_FILES := \
	$(LAMBDA_FUNCTION_DIR)/lambda_function.py \
	$(LAMBDA_FUNCTION_DIR)/lambda_function.test.py \
	$(LAMBDA_FUNCTION_DIR)/pyproject.toml \
	$(LAMBDA_FUNCTION_DIR)/event.json

.PHONY: local-clean
local-clean:
	@echo "Cleaning $$(pwd)"
	@rm -rf $(LAMBDA_FUNCTION_PACKAGE_DIR) dist .venv
	@rm -f $(LAMBDA_FUNCTION_ARTIFACT_ZIP) poetry.lock

.PHONY: local-poetry-env-list
local-poetry-env-list: poetry-check
	$(POETRY_BIN) env list

.PHONY: local-poetry-update
local-poetry-update: poetry-update

.PHONY: local-poetry-install
local-poetry-install: poetry-install

.PHONY: local-poetry-build
local-poetry-build: local-poetry-install
	@printf "$(bold)Build lambda function $(green)$(LAMBDA_FUNCTION_NAME)$(normal)\n"
	set -x ; cd $(LAMBDA_FUNCTION_DIR) && $(POETRY_BIN) build

.PHONY: local-poetry-test
local-poetry-test: local-poetry-build
	cd $(LAMBDA_FUNCTION_DIR) && $(POETRY_BIN) run pytest

.PHONY: local-poetry-run
local-poetry-run: local-poetry-install
	cd $(LAMBDA_FUNCTION_DIR) && $(POETRY_BIN) run --help

$(LAMBDA_FUNCTION_PACKAGE_DIR):
	@mkdir -p $@

$(LAMBDA_FUNCTION_ARTIFACT_ZIP): $(LAMBDA_FUNCTION_SOURCE_FILES) $(LAMBDA_FUNCTION_PACKAGE_DIR) zip-check
	$(MAKE) -C $(LAMBDA_FUNCTION_DIR) -f Makefile local-poetry-build
	@printf "$(bold)Package lambda function $(green)$(LAMBDA_FUNCTION_NAME)$(normal)\n"
	@cd $(LAMBDA_FUNCTION_DIR) && \
		$(POETRY_BIN) run $(PYTHON_BIN) -m pip install --upgrade -t $(LAMBDA_FUNCTION_PACKAGE_DIR) dist/*.whl
	@cd $(LAMBDA_FUNCTION_PACKAGE_DIR) ; $(ZIP_BIN) -r $@ . -x '*.pyc'

.PHONY: dist
dist: $(LAMBDA_FUNCTION_ARTIFACT_ZIP)

local-test: lambda_function.test.py
	cd $(LAMBDA_FUNCTION_DIR) && $(PYTHON_BIN) $? -v

local-aws-logs: event.json
	export AWS_REGION="$$($(SOPS_BIN) -d --extract '["AWS_REGION"]' $(GIT_ROOT)/keys.json)" && \
	export AWS_ACCESS_KEY_ID="$$($(SOPS_BIN) -d --extract '["AWS_ACCESS_KEY_ID"]' $(GIT_ROOT)/keys.json)" && \
	export AWS_SECRET_ACCESS_KEY="$$($(SOPS_BIN) -d --extract '["AWS_SECRET_ACCESS_KEY"]' $(GIT_ROOT)/keys.json)" && \
	echo "Invoke lambda function $(LAMBDA_FUNCTION_NAME):" && \
	aws lambda invoke \
		--function-name $(LAMBDA_FUNCTION_NAME) \
		--cli-binary-format raw-in-base64-out \
		--payload file://event.json \
		 out && \
	echo "Result:" && \
	cat out | jq && \
	sed -i'' -e 's/"//g' out && \
	sleep 15 && \
	aws logs get-log-events \
		--log-group-name /aws/lambda/$(LAMBDA_FUNCTION_NAME) \
		--log-stream-name stream1 \
		--limit 5
