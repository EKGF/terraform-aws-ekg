ifndef GIT_ROOT
GIT_ROOT := $(shell git rev-parse --show-toplevel 2>/dev/null)
endif

MK_DIR := $(GIT_ROOT)/.make

-include $(GIT_ROOT)/ekgf-make.mk

.PHONY: build
build: cargo-check cargo-lambda-check
	$(CARGO_BIN) +nightly lambda build --arm64 --release

.PHONY: test
test: cargo-check
	RUST_LOG=info $(CARGO_BIN) test

.PHONY: watch
watch: cargo-check cargo-lambda-check
	AWS_REGION=eu-west-2 \
	AWS_NEPTUNE_LOAD_IAM_ROLE_ARN=arn:aws:iam::457852604093:role/ekgf-dt/dev/staging/ekgf-dt-dev-staging-neptune \
	EKG_API_BASE=https://dev.digital-twin.ekgf.org/api \
	EKG_BASE_EXTERNAL=https://dev.digital-twin.ekgf.org \
	EKG_BASE_INTERNAL=https://placeholder.kg \
	EKG_GRAPH_BASE_EXTERNAL=https://dev.digital-twin.ekgf.org/graph \
	EKG_GRAPH_BASE_INTERNAL=https://placeholder.kg/graph \
	EKG_ID_BASE_EXTERNAL=https://dev.digital-twin.ekgf.org/id \
	EKG_ID_BASE_INTERNAL=https://placeholder.kg/id \
	EKG_ONTOLOGY_BASE_EXTERNAL=https://dev.digital-twin.ekgf.org/ontology \
	EKG_ONTOLOGY_BASE_INTERNAL=https://placeholder.kg/ontology \
	EKG_SPARQL_HEALTH_ENDPOINT=https://ekgf-dt-dev-staging.cluster-ro-ckvyb9qgnfdp.eu-west-2.neptune.amazonaws.com:8182/status \
	EKG_SPARQL_LOADER_ENDPOINT=https://ekgf-dt-dev-staging.cluster-ro-ckvyb9qgnfdp.eu-west-2.neptune.amazonaws.com:8182/loader \
	EKG_SPARQL_QUERY_ENDPOINT=https://ekgf-dt-dev-staging.cluster-ro-ckvyb9qgnfdp.eu-west-2.neptune.amazonaws.com:8182/sparql \
	EKG_SPARQL_UPDATE_ENDPOINT=https://ekgf-dt-dev-staging.cluster-ckvyb9qgnfdp.eu-west-2.neptune.amazonaws.com:8182/sparql \
	neptune_s3_bucket_region=eu-west-2 \
	neptune_s3_iam_role_arn=arn:aws:iam::457852604093:role/ekgf-dt/dev/staging/ekgf-dt-dev-staging-neptune \
	$(CARGO_BIN) +nightly lambda watch --wait -vv

.PHONY: invoke
invoke: cargo-check cargo-lambda-check event.json
	$(CARGO_BIN) +nightly lambda invoke --data-file event.json -vv