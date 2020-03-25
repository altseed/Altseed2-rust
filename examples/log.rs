//! # ログを出力するサンプル
use altseed2::prelude::*;

fn main() -> AltseedResult<()> {
    // Altseedを初期化します。 ?演算子を利用してError時に早期終了します。
    let mut engine = Engine::initialize("log", 800, 600)?;

    engine
        .log()
        .info("コンソールとファイルにログを出力できます");

    engine.log().warn("重要度に応じて種類を変更できます");

    engine.log().trace("トレース");
    engine.log().info("情報");
    engine.log().debug("デバッグ");
    engine.log().error("エラー");
    engine.log().critical("致命的");

    // 正常終了
    Ok(())
    // engineがdropする際に自動的にAltseedの終了処理が呼ばれます。
}
