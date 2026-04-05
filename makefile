LIB_NAME := librelation.so
LIB_HEADER := librelation.h
SRC_DIR := go_methods
BINARIES := relation relationd

PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin
LIBDIR ?= $(PREFIX)/lib

SINGBOX_TAGS := with_dhcp with_quic with_utls with_acme with_gvisor with_tailscale
TAGS := linux $(SINGBOX_TAGS)

.PHONY: build release install install-files clean test

build:
	CGO_ENABLED=1 go build -tags "$(TAGS)" -buildmode=c-shared -o ./$(LIB_NAME) ./$(SRC_DIR)
	cargo build --bins
	cp target/debug/relation .
	cp target/debug/relationd .

release:
	CGO_ENABLED=1 go build -tags "$(TAGS)" -ldflags "-s -w" -buildmode=c-shared -o ./$(LIB_NAME) ./$(SRC_DIR)
	cargo build --release --bins
	cp target/release/relation .
	cp target/release/relationd .

install-files:
	install -Dm755 target/release/relation $(DESTDIR)$(BINDIR)/relation
	install -Dm755 target/release/relationd $(DESTDIR)$(BINDIR)/relationd
	install -Dm755 $(LIB_NAME) $(DESTDIR)$(LIBDIR)/$(LIB_NAME)
	install -Dm644 README.md $(DESTDIR)$(PREFIX)/share/doc/relation/README.md

install: release install-files

clean:
	rm -f $(LIB_NAME) $(LIB_HEADER) $(BINARIES)

test:
	go test -tags "$(TAGS)" ./... -v
