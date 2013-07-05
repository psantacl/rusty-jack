RUSTC ?= rustc

all: audio.rs rusty-jack.rc
	$(RUSTC) --lib rusty-jack.rc

clean:
	rm -rf *.dylib *.dSYM
