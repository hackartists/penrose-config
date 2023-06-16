REL=target/release/penrose-config

all: clean install

test:
	cargo test

build: test
	cargo build --release

$(REL): build

install: $(REL)
	cp -rf $(REL) $(HOME)/.local/bin/penrose-config

clean:
	rm -rf target

