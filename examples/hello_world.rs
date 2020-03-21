//! # ウィンドウを表示するサンプル
use altseed2::prelude::*;

// add-timer
// 返り値に`AltseedResult<()>`を指定して?演算子を使用可能にします。
fn main() -> AltseedResult<()> {
    // Altseedを初期化します。 ?演算子を利用してError時に早期終了します。
    let engine = Engine::initialize("test", 800, 600)?;

    // ここで画像などのデータを読み込んだりノードツリーを作成したりすることができます。

    // 所有権を渡してメインループを実行します。
    engine.run()?;
    // engineがdropする際に自動的にAltseedの終了処理が呼ばれます。

    // 正常終了
    Ok(())
}
