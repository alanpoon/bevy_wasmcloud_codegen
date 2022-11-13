# bevy_wasmcloud/weld top-level Makefile

gen:
	rm -rf dist
	mkdir dist
	cp -r templates/actors dist
	cp -r templates/www dist
	cp -r templates/shared dist
	cp -r templates/Cargo.toml dist
.PHONY: all
.NOTPARALLEL:
