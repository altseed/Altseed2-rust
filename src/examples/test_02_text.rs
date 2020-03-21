// Auto-generated. Do not modify.
// このファイルは自動生成されたものです。変更しないでください。
//! ```
//! use altseed2::prelude::*;
//!
//! include!("../tests/timer.rs");
//! fn main() -> AltseedResult<()> {
//!     // Altseedを初期化します。
//!     let engine = Engine::initialize("test", 800, 600)?;
//!     engine.add_node(timer::TimerNode::new(10))?;
//!
//!     let font = engine
//!         .loader()
//!         .load_dynamic_font("./Core/TestData/Font/mplus-1m-regular.ttf", 50)?;
//!
//!     let node = {
//!         Text::new()
//!             .with_font(font)
//!             .with_text("Hello, world! こんにちは")
//!             .build()
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
//!     // engineがdropする際に自動的にAltseedの終了処理が呼ばれます。
//! }
