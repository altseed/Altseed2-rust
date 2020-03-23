// Auto-generated. Do not modify.
// このファイルは自動生成されたものです。変更しないでください。

//! ```
//! use altseed2::prelude::*;
//!
//! fn main() -> AltseedResult<()> {
//!     // Altseedを初期化します。 ?演算子を利用してError時に早期終了します。
//!     let engine = Engine::initialize("log", 800, 600)?;
//!
//!     engine
//!         .log()
//!         .borrow_mut()
//!         .info("コンソールとファイルにログを出力できます");
//!
//!     engine
//!         .log()
//!         .borrow_mut()
//!         .warn("重要度に応じて種類を変更できます");
//!
//!     engine.log().borrow_mut().trace("トレース");
//!     engine.log().borrow_mut().info("情報");
//!     engine.log().borrow_mut().debug("デバッグ");
//!     engine.log().borrow_mut().error("エラー");
//!     engine.log().borrow_mut().critical("致命的");
//!
//!     // 正常終了
//!     Ok(())
//!     // engineがdropする際に自動的にAltseedの終了処理が呼ばれます。
//! }
