RUSTC = rustc
TARGET = wasm32-unknown-emscripten
OPTION := --target=$(TARGET)
LINK_ARGS = -s EXPORTED_FUNCTIONS=['_raytrace1','_raytrace2','_hello']


.PHONY: clean all server
all: clean trace.wasm

clean:
	rm -f *.js *.wasm

trace.wasm:
	rustc $(OPTION) trace.rs -C link-args="$(LINK_ARGS)"

server:
	python -m SimpleHTTPServer
