MK_TAR_DIR := $(HOME)/.tmp
MK_TAR := $(MK_TAR_DIR)/make.tar.gz
MK_DIR := .make
MK_URL := https://github.com/EKGF/make/archive/refs/heads/main.tar.gz
MK_FLAG_FILE := $(MK_DIR)/os.mk
.PRECIOUS: $(MK_FLAG_FILE)

CURL_BIN := $(shell command -v curl 2>/dev/null)
ifndef CURL_BIN
$(error curl not installed)
endif

include $(MK_FLAG_FILE)
-include $(MK_DIR)/*.mk

$(MK_DIR):
	@echo "Creating the $(MK_DIR) directory"
	@mkdir -p $(MK_DIR) >/dev/null 2>&1

$(MK_TAR_DIR):
	@echo "Creating the $(MK_TAR_DIR) directory"
	@mkdir -p $(MK_TAR_DIR) >/dev/null 2>&1

$(MK_TAR): $(MK_TAR_DIR)
	@echo "Downloading $@"
	@$(CURL_BIN) -L -s -S -f -o $@ --url $(MK_URL)

$(MK_FLAG_FILE): $(MK_DIR) $(MK_TAR)
	@echo "Extracting the EKGF Make files into the $(MK_DIR) directory"
	@tar -xzf $(MK_TAR) -C $(MK_DIR) --strip-components=1
	@rm -rf $(MK_DIR)/.idea
	@touch -mc $(MK_DIR)/*
	-@$(MAKE) --no-print-directory $(MAKECMDGOALS)

#$(MK_DIR)/*.mk &: $(MK_FLAG_FILE)

.PHONY: mk-clean
mk-clean:
	@echo "mk-clean"
	@rm -f $(MK_TAR)
	@rm -rf $(MK_DIR)
