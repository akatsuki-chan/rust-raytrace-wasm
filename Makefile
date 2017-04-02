RUSTC = rustc
TARGET = wasm32-unknown-emscripten
OPTION := --target=$(TARGET)
LINK_ARGS = -s EXPORTED_FUNCTIONS=['_raytrace1','_raytrace2','_hello']


all: clean trace.wasm

.PHONY: clean
clean:
	rm -f *.js *.wasm

trace.wasm: trace.rs
	rustc $(OPTION) trace.rs -C link-args="$(LINK_ARGS)"

server:
	python -m SimpleHTTPServer