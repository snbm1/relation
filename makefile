# var names 
PROJECT := methods 
OUT := build
LIB_NAME := librelation.so
SRC_DIR := go_methods

#Sing-Box tags 
SINGBOX_TAGS := with_grpc with_dhcp with_quic with_utls with_acme with_gvisor with_tailscale
TAGS := linux $(SINGBOX_TAGS)

build: 
	CGO_ENABLED=1 go build -tags "$(TAGS)" -buildmode=c-shared -o ./$(LIB_NAME) ./$(SRC_DIR)


release: 
	CGO_ENABLED=1 go build -tags "$(TAGS)" -ldflags "-s -w" -buildmode=c-shared -o ./$(LIB_NAME) ./$(SRC_DIR)

test: 
	go test -tags "$(TAGS)" ./... -v 

clean: 
	rm -fr $(OUT)
