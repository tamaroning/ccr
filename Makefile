# CCの場合は-staticを指定する必要がある?

ccr: src/main.rs src/codegen.rs src/parse.rs src/tokenize.rs
	rustc -o ccr src/main.rs

test: ccr
	./test.sh

clean:
	rm -f ccr tmp*
	rm -f ccr *.out
	rm -f ccr *.o
	rm -f ccr *.out

#以下のターゲットはファイルを生成しない
.PHONY: clean test
