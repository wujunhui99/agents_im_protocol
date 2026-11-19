PROTOC  ?= protoc
MODULE  := github.com/wujunhui99/agents_im_protocol
PROTOS  := $(shell find proto -name '*.proto' | sort)

.PHONY: gen verify build

gen:
	$(PROTOC) --go_out=. --go_opt=module=$(MODULE) \
	          --go-grpc_out=. --go-grpc_opt=module=$(MODULE) $(PROTOS)
	go mod tidy

verify:
	@$(MAKE) --no-print-directory gen
	@git diff --exit-code -- '*.pb.go' go.mod go.sum || \
	  (echo "generated code is stale: run 'make gen' and commit"; exit 1)

build:
	go build ./...
