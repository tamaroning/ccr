# CCの場合は-staticを指定しなきゃいけないらしい
#

ccr: src/main.rs
	rustc -o ccr src/main.rs

test: ccr
	./test.sh

clean:
	rm -f ccr tmp*

#以下のてターゲットはファイルを生成しない
.PHONY: clean test
