
// Auto-generated. Do not modify.
// このファイルは自動生成されたものです。変更しないでください。

//! # ログを出力するサンプル
//!
//! ```

//! 
//! use altseed2::prelude::*;
//! 
//! fn main() -> AltseedResult<()> {
//!     // Altseedを初期化します。 ?演算子を利用してError時に早期終了します。
//!     let engine = Engine::initialize("engine", 800, 600)?;
//! 
//!     engine.log().borrow_mut().info(
//!         LogCategory::User,
//!         "ログ機能を使ってコンソールとファイルにログを出力できます",
//!     );
//!     engine.log().borrow_mut().warn(
//!         LogCategory::User,
//!         "重要度に応じてログの種類を切り替えられます",
//!     );
//!     engine
//!         .log()
//!         .borrow_mut()
//!         .trace(LogCategory::User, "トレース");
//!     engine.log().borrow_mut().info(LogCategory::User, "情報");
//!     engine
//!         .log()
//!         .borrow_mut()
//!         .debug(LogCategory::User, "デバッグ");
//!     engine.log().borrow_mut().error(LogCategory::User, "エラー");
//!     engine
//!         .log()
//!         .borrow_mut()
//!         .critical(LogCategory::User, "致命的");
//! 
//!     // 正常終了
//!     Ok(())
//! }
//! 

//! ```
//!
//! ## Run this example
//! ```shell
//! cargo run --example log
//! ```

