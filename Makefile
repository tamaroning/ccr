# CCの場合は-staticを指定する必要がある?

ccr: src/main.rs
	rustc -o ccr src/main.rs

test: ccr
	./test.sh

clean:
	rm -f ccr tmp*

#以下のターゲットはファイルを生成しない
.PHONY: clean test
