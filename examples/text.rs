//! # 文字列を表示するサンプル
use altseed2::prelude::*;

// add-timer
fn main() -> AltseedResult<()> {
    // Altseedを初期化します。
    let engine = Engine::initialize("test", 800, 600)?;

    let font = engine
        .loader()
        .load_dynamic_font("./Core/TestData/Font/mplus-1m-regular.ttf", 50)?;

    let node = {
        Text::new()
            .with_font(font)
            .with_text("Hello, world! こんにちは")
            .build()
    };

    engine.add_node(node)?;

    // メインループを実行します。
    engine.run()?;

    // 正常終了
    Ok(())

    // engineがdropする際に自動的にAltseedの終了処理が呼ばれます。
}
