# ccr
Ccr is a toy C compiler written in Rust language, which I am making in order to understand compilers and Rust.  
This software supports a small subset of C and outputs x64 GNU assembler.  
Ccr is not so fast, but the source is easy for beginners to read.   


# Usage
## Build ccr
To use ccr, run the following scripts:

```sh
$ git clone https://github.com/tamaroning/ccr
$ cd ccr
$ cargo build
```


## Run
After the build, Compile text(.c) file:  

```sh
$ ./ccr [file path]
```
By default, the assembly is written to tmp.s.  
  
Then assemble tmp.s into an executable binary:  

```sh
$ cc -o tmp tmp.s
$ ./tmp
```

Check the return value:  

```sh
$ echo $?
```
Make sure that the return value is cure in the range of 0~255  


## Test
You can run test.sh to execute the demo:  

```sh
$ ./test.sh
```


# Implemented features
- Numeric literals (only signed integer) (ex: 0, 24, +4, -699)
- Basic arithmetic operators (+, -, *, /)
- Dereference and address operators (*, &)
- Comparison operators (==, !=, </>, <=/>=)
- Local variables (need to be declared)
- Variable declaration and initialization (ex: int a, b = 0;)
- Return statement
- Assignment (ex: a = 4*3;)
- Control syntax (if-else, for, while)


## Example 1
Ccr can compile programs like the following: 

```c
int sum = 0;
int width = 3;
for ( i = 1; i <= width; i = i + 1) {
    for (j = 1; j <= width; j = j + 1) {
        sum = sum + 1;
    }
}

if (sum == 9) return 1;
else return 0;
```

## Example 2
You can use functions which is defined in other object files:  
```c
int i;
for(i = 0; i < 10; i = i + 1) {
    foo();
}
```
In this case, foo() is defined in an .o file.
And you need to link the .o file to the assembly dumped by ccr.  


# Todo
## Steps  
- [x] Step3 簡単な式(例: 3+12-5)の結果を出力する
- [x] Step5-1 EBNFによる文法の定義と再帰下降構文解析
- [x] Step5-2 スタックマシンへのコンパイル
- [x] Step6 単項プラス/マイナス
- [x] Step7 比較演算子
- [x] Step9,10 ローカル変数
- [x] Step11 return文
- [x] Step12 if-else, for, while
- [x] Step13 {}ブロック
- [x] Step14 関数呼び出し
- [x] 単項*と& (derefとaddr)
    - 現在はbyte単位での演算が可能
- [ ] 型 (int, int*, int**, ...)
    - 次にsizeofを実装する必要がありそう
- [ ] 定義,宣言
    - intのみ対応
    - ポインタ型の実装と並行して進める
    - int a = 0, b = 1; は{a=0;b=0}と等価の出力を行うのでスコープの実装後に修正が必要
- [ ] Step15 関数定義


##  Refactoring and improvement
- [x] Refactoring: TokenizeとParseの処理を分ける  
- [x] .cファイルを読み込んで.sファイルを吐き出せるようにする
- [x] 標準出力に実行中の情報を出力できるようにする
- [ ] コード生成時に検出されるエラー出力の強化


## Issues
- [x] for(;;){}を受け付けるようにexpr=Nilを許容する
    - 後ろに;が続く場合のみ許容
- [ ] for(int i = 0;;)のようにfor内で変数の定義ができるようにする
- [ ] 関数呼び出し時のスタックフレームの確保(スタックフレームサイズの把握)とretの数を修正する
- [ ] ポインタの演算の修正 (intのポインタpにNを足すと,N要素先のintを指すようにする, つまり+/-演算子のオーバーロード)
- [ ] 6つ以上の引数の関数呼び出し
- [ ] EBNFの修正 C言語の正しい文法にする
- [ ] 変数スコープの実装


# Internals
Ccr is designed with reference to chibicc.  
If you want to laern the internals, please read [chibicc internals](https://github.com/rui314/chibicc#internals)  


# References
0. [低レイヤを知りたい人のためのCコンパイラ作成入門](https://www.sigbus.info/compilerbook#), Rui Ueyama
1. [chibicc: A Small C Compiler](https://github.com/rui314/chibicc), rui314
2. [Compilers: Principles, Techniques, and Tools](https://www.amazon.com/Compilers-Principles-Techniques-Tools-2nd/dp/0321486811), Alfred V.Aho, Monica S.Lam, Ravi Sethi, and Jeffrey D.Ullman
3. [A Grammar for the C- Programming Language](http://marvin.cs.uidaho.edu/Teaching/CS445/c-Grammar.pdf), Robert Heckendorn
