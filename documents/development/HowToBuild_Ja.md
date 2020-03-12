## Core
[CoreのHowToBuild_Ja.md](Core/../../../Core/documents/development/HowToBuild_Ja.md)を参照

## 必須
- [rustup](https://rustup.rs) 1.21.1 (13979c968 2019-10-16)

## サブモジュール取得

```shell
$ git submodule update --init
```

## Build
```shell
$ cargo build # Debug
$ cargo build --release # Release
```

## Format
```shell
$ cargo fmt
```

(`rustup component add rustfmt` to install)

## Binding生成 (commitするのでmaintainerが行う)
[CppBindingGenerator](/Core/scripts/bindings/CppBindingGenerator)を更新したら実行する
```shell
$ python scripts/copy_cbg.py
```

[Core/scripts/bindings](/Core/scripts/bindings)を更新したら実行する
```
python scripts/generate_binding.py
```
