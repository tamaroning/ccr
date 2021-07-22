# ccr : C Compiler written in Rust
Ccr is a toy compiler of Rust language, which I am making in order to understand compilers and Rust.

# Usage

## Compile
To use ccr, run the following scripts:
```
$ git clone https://github.com/tamaroning/ccr
$ cd ccr
$ make
```

## Run
```
$ ./ccr [expr] > tmp.s
$ cc -o tmp tmp.s
$ ./tmp
$ echo $?
```

## Test
```
$ ./test.sh
```

# Todo
- [x] Step3 簡単な式(例: 3+12-5)の結果を出力する
- [x] Step4 エラーメッセージの改良
- [x] Step5-1 EBNFによる文法の定義と再帰下降構文解析
- [ ] Step5-2 スタックマシンへのコンパイル
- [ ] Step5-3 四則演算のできる言語の作成
- [ ] Step6 単項プラス/マイナスの実装
- [ ] Step7 比較演算子

StepNはBibliography[0]を参照

# Bibliography
[0] 低レイヤを知りたい人のためのCコンパイラ作成入門, (https://www.sigbus.info/compilerbook#)

