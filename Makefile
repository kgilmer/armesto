#/usr/bin/make -f

INSTALL_PROGRAM = install -D -m 0755
INSTALL_DATA = install -D -m 0644

prefix = ${DESTDIR}/usr
exec_prefix = $(prefix)
bindir = $(exec_prefix)/bin
datarootdir = $(prefix)/share
libdir = $(exec_prefix)/lib
zshcpl = $(datarootdir)/zsh/site-functions

BIN := armesto

MESON = meson


all: build

distclean: clean

clean:
	-cargo clean

build-arch: build

build-independent: build

binary: build

binary-arch: build

binary-independent: build

build: 
	cargo build --release

install: 
	$(INSTALL_PROGRAM) "./target/release/$(BIN)" "$(bindir)/$(BIN)"

uninstall:
	rm -f "$(bindir)/$(BIN)"

run-test:
	cargo test -- --test-threads=1
