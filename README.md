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
- [x] Step5-2 スタックマシンへのコンパイル
- [ ] Step6 単項プラス/マイナス
- [ ] Step7 比較演算子
- [ ] Step8 ソースコード分割
- [ ] Step9 1文字のローカル変数
- [ ] Step10 複数文字のローカル変数
- [ ] Step11 return文
- [ ] Step12 制御構文
- [ ] Step13 {}ブロック
- [ ] Step14 関数呼び出し
- [ ] Step15 関数定義


StepNはBibliography[0]を参照

# Bibliography
[0] 低レイヤを知りたい人のためのCコンパイラ作成入門, (https://www.sigbus.info/compilerbook#)

