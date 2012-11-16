target/hulk: src/hulk.rc $(wildcard src/*.rs)
	rustc -g --out-dir target src/hulk.rc

clean:
	rm -fr target/*

target/hulk_test: src/hulk.rc $(wildcard src/*.rs)
	rustc -g --test -o target/hulk_test src/hulk.rc
