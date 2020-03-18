
// Auto-generated. Do not modify.
// このファイルは自動生成されたものです。変更しないでください。

//! # 文字列を表示するサンプル
//!
//! ```

//! 
//! use altseed2::prelude::*;
//! 
//! fn main() -> AltseedResult<()> {
//!     // Altseedを初期化します。
//!     let mut engine = Engine::initialize("engine", 800, 600)?;
//! 
//!     let font = engine
//!         .loader()
//!         .load_dynamic_font("./Core/TestData/Font/mplus-1m-regular.ttf", 50)?;
//! 
//!     let node = {
//!         let text = Text::new();
//!         text.borrow_mut()
//!             .set_font(&font)
//!             .set_text("Hello, world! こんにちは");
//!         DrawnNode::new(text)
//!     };
//! 
//!     engine.add_node(node)?;
//! 
//!     // メインループを実行します。
//!     engine.run()?;
//! 
//!     // 正常終了
//!     Ok(())
//! 
//!     // engine が dropする際に自動的にAltseedの終了処理が呼ばれます。
//! }
//! 

//! ```
//!
//! ## Run this example
//! ```shell
//! cargo run --example text
//! ```

