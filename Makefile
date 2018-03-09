CARGO = cargo

all:
	$(MAKE) build

build:
	$(CARGO) $(CARGO_OPTS) build

clean:
	$(CARGO) $(CARGO_OPTS) clean

.PHONY: all build clean
