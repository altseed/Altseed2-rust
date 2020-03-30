# Engineについて

Altseedの[Engine](../../engine/struct.Engine.html)には以下の役割があります。

1. Altseedの[Core](../../core/index.html)機能へアクセスする。
2. 描画機能の管理を行う。

以下はRust版Altseedでの最も簡単なサンプルです。

```no_run
use altseed2::prelude::*;

fn main() -> AltseedResult<()> {
    let engine = Engine::initialize("test", 800, 600)?;

    engine.run()?;

    Ok(())
}
```

## 処理の流れ

1. [Engine::initialize](../../engine/struct.Engine.html#method.initialize)を実行して[Engine](../../engine/struct.Engine.html)構造体を作成します。
1. [Engine::run](../../engine/struct.Engine.html#method.run)を実行してAltseedのメインループを実行します。この関数は`self`を要求するので、`engine`はムーブされてメインループが終了する際に開放されます。
1. [Drop](https://doc.rust-lang.org/std/ops/trait.Drop.html)トレイトによって自動的に`Terminate`処理が行われます。 

メインループ内で実行したい処理は[Engine::run_with](../../engine/struct.Engine.html#method.run)の引数に関数を渡して記述できます。

また、以下のように再び所有権を受け取ることでウィンドウ終了後の処理を記述可能です。

```ignore
let engine = engine.run()?;

engine.something();
```

## 非同期処理
[Future](https://doc.rust-lang.org/beta/std/future/trait.Future.html)(async/await)に対応しています。

[Engine::spawn_task](../../engine.struct.Engine.html#method.spawn_task)を利用して非同期処理を実行することができます。
([Examples/load_async](../../examples/_06_load_async.rs))

継続処理はメインループ内で取り出されます。
これは`spawn_task`が`Send + Sync`を要求しないことからもわかります。

非同期処理の実行後に`Engine`の機能にアクセスしたい場合は[Cont](../../task/enum.Cont.html)(Continuation, 継続)を用いて以下のように記述します。

```ignore
async {
    // 処理

    // 返り値で継続を指定
    // 必要ない場合は`Cont::fin()`
    Cont::then(|e: &mut Engine| {
        e.hoge();
        Ok(())
    })
}
```

## C#版との違い

### Engineの使い方
1. [Engine](../../engine/struct.Engine.html)がstaticではない。
2. メインループが[Engine::run](../../engine/struct.Engine.html#method.run)という関数になっている。
3. `Terminate`が自動的に呼ばれる。

1と3に関しては、Rustでは[Engine](../../engine/struct.Engine.html)が初期化されて終了されるまでの範囲をライフタイムとして表すことができるため、こういった実装にしました。
また、`Rc<RefCell<T>>`で扱うのはかなり注意が必要になるといった事情もあります。

2に関しては、`do_events`と`update`は更新順序が関係して本質的に`unsafe`なので隠蔽しました。
処理を記述したい場合は、代わりに[Engine::run_with](../../engine/struct.Engine.html#method.run)を利用することができます。

### ファイルを読み込む
[Loader](../../engine/struct.Loader.html)に別れています。
[Engine::loader](../../engine/struct.Engine.html#method.loader)を使って`Loader`を得られます。

```ignore
engine.loader().load_texture2d("hoge.png")?;
```

のように記述可能です。([Examples/load_async](../../examples/_06_load_async.rs))
マルチスレッドで安全に扱うための仕様です。
