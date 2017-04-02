# rust de raytrace de webassembly

要： rustでwasmを作り出せる環境

## 実験元環境
1. Chrome 57.0.〜
1. MacOSX 10.10.5 Yosemite
1. Rust 1.15.0
1. うぇっぶさーばー（python -m SimpleHTTPServerでも可）

## 手順

```
$ git clone https://github.com/akatsuki-chan/rust-raytrace-wasm.git
$ cd rust-raytrace-wasm
$ make
$ make server
```

↑でサーバー起動後に、127.0.0.1:8000で確認。
（要webassemblyが利用できるブラウザ）

## EXPORTED FUNCTIONS

- hello

ブラウザのconsoleにメッセージ出すだけ

- raytrace1

rustでメモリ作ってレンダリングするバージョン

- raytrace2

javascriptでメモリ確保してレンダリングするバージョン
