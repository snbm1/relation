PROJECT := methods 
LIB_NAME := librelation.so
SRC_DIR := go_methods

SINGBOX_TAGS := with_grpc with_dhcp with_quic with_utls with_acme with_gvisor with_tailscale
TAGS := linux $(SINGBOX_TAGS)

clean: 
	rm librelation.*
	rm relation

build: clean
	CGO_ENABLED=1 go build -tags "$(TAGS)" -buildmode=c-shared -o ./$(LIB_NAME) ./$(SRC_DIR)
	cargo build
	cp target/debug/relation .


release: clean
	CGO_ENABLED=1 go build -tags "$(TAGS)" -ldflags "-s -w" -buildmode=c-shared -o ./$(LIB_NAME) ./$(SRC_DIR)
	cargo build --release
	cp target/release/relation .

test: 
	go test -tags "$(TAGS)" ./... -v 

