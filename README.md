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
```

Check the return value:  
```
$ echo $?
```
Make sure that the return value is cure in the range of 0~255  

## Test
You can run test.sh to execute the demo:  
```
$ ./test.sh
```

# Implemented features
- Numeric literals (only signed integer) (ex: 0, 24, +4, -699)
- Basic arithmetic operatiors (+, -, *, /)
- Comparison operators (==, !=, </>, <=/>=)
- Local variables (No need to declare)
- Return statement
- Assignment (ex: a = 4*3;)
- Control sytax (if-else, for, while)

## Example 1 )  
Ccr can compile programs like the following:  
```
sum = 0;
width = 3;
for ( i = 1; i <= width; i = i + 1) {
    for (j = 1; j <= width; j = j + 1) {
        sum = sum + 1;
    }
}

if (sum == 9) return 1;
else return 0;
```
## Example 2 )
You can use functions which is defined in other object files:  
```
for(i = 0; i < 10; i = i + 1) {
    foo();
}
```
In this case, foo() is defined in an .o file.
And you need to link the .o file to the assembly dumped by ccr.  

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
- [x] Step12-1 if-else
- [x] Step12-2 while
- [x] Step12-3 for
- [x] Step13 {}ブロック
- [x] Step14-1 引数なし関数呼び出し
- [ ] Step14-2 引数あり関数呼び出し
- [ ] Step15 関数定義

##  Refactoring and improvement
- [x] Refactoring: TokenizeとParseの処理を分ける  
- [x] .cファイルを読み込んで.sファイルを吐き出せるようにする
- [x] 標準出力に実行中の情報を出力できるようにする
- [ ] コード生成時に検出されるエラー出力の強化
- [ ] 関数呼び出し時のスタックフレームの確保(スタックフレームサイズの把握)とretの数を修正する

# References
0. Rui Ueyama. 低レイヤを知りたい人のためのCコンパイラ作成入門. (https://www.sigbus.info/compilerbook#)  
1. rui314. chibicc: A Small C Compiler. (https://github.com/rui314/chibicc)  
