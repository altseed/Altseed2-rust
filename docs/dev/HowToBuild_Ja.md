## Core
[CoreのHowToBuild_Ja.md](Core/../../../Core/documents/development/HowToBuild_Ja.md)を参照

## 必須
- [rustup](https://rustup.rs) 1.21.1 (13979c968 2019-10-16)

## サブモジュール取得

```shell
$ git submodule update --init
```

## ビルド
```shell
$ cargo build # Debug
$ cargo build --release # Release
```

## フォーマット
コミットの前に実行する

```shell
$ cargo fmt
```

インストール: `rustup component add rustfmt`

## Example生成
`examples/`以下を編集したら実行する

```shell
$ python scripts/generate_example.py
```

## Rust Docs生成

```shell
$ cargo rustdoc
```

なお、 `docs/contents`以下のMarkdownファイルを編集したら以下を実行する

```shell
$ python scripts/generate_docs.py

```

## Binding生成
commitするのでmaintainerが行う

```shell
$ python scripts/copy_cbg.py
$ python scripts/generate_binding.py
```
