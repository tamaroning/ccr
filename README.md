# ccr : C Compiler written in Rust
Ccr is a toy C compiler written in Rust language, which I am making in order to understand compilers and Rust.  
This software supports a small subset of C and outputs GNU assembler.  
Ccr is not so fast, but the source is easy for beginners to read.   

# Usage
## Compile ccr
To use ccr, run the following scripts:
```
$ git clone https://github.com/tamaroning/ccr
$ cd ccr
$ make
```

## Run
Compile text(.c) file:  
```
$ ./ccr [file path]
```
By default, the assembly is written to tmp.s.  
  
Then assemble tmp.s into an executable binary:  
```
$ cc -o tmp tmp.s
$ ./tmp
$ echo $?
```

## Test
You can run test.sh to execute the demo:  
```
$ ./test.sh
```

# Todo
## Steps  
- [x] Step3 簡単な式(例: 3+12-5)の結果を出力する
- [x] Step4 エラーメッセージの改良
- [x] Step5-1 EBNFによる文法の定義と再帰下降構文解析
- [x] Step5-2 スタックマシンへのコンパイル
- [x] Step6 単項プラス/マイナス
- [x] Step7 比較演算子
- [x] Step8 ソースコード分割
- [x] Step9 1文字のローカル変数
- [x] Step10 複数文字のローカル変数
- [x] Step11 return文
- [ ] Step12 制御構文
- [ ] Step13 {}ブロック
- [ ] Step14 関数呼び出し
- [ ] Step15 関数定義

refs: Bib[0]

##  Refactoring and improvement
- [x] Refactoring: TokenizeとParseの処理を分ける  
- [x] .cファイルを読み込んで.sファイルを吐き出せるようにする
- [x] 標準出力に実行中の情報を出力できるようにする
- [ ] コード生成時に検出されるエラー出力の強化 
- [ ] 関数呼び出し時のスタックフレームの確保(スタックフレームサイズの把握)とretの数を修正する

# Bibliography
[0] Rui Ueyama, '低レイヤを知りたい人のためのCコンパイラ作成入門', (https://www.sigbus.info/compilerbook#)

