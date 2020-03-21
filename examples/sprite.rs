//! # 画像を表示するサンプル
use altseed2::prelude::*;

// add timer
fn main() -> AltseedResult<()> {
    // Altseedを初期化します。
    let engine = Engine::initialize("sprite", 800, 600)?;

    let tex = engine
        .loader()
        .load_texture2d("./Core/TestData/IO/AltseedPink256.png")?;

    let node = Sprite::new().with_tex(tex).build();

    engine.add_node(node)?;

    // メインループを実行します。
    engine.run()?;

    // 正常終了
    Ok(())

    // engineがdropする際に自動的にAltseedの終了処理が呼ばれます。
}
