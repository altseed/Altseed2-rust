// Auto-generated. Do not modify.
// このファイルは自動生成されたものです。変更しないでください。

//! ```
//! use altseed2::prelude::*;
//!
//! include!("../tests/timer.rs");
//! fn main() -> AltseedResult<()> {
//!     // Altseedを初期化します。
//!     let engine = Engine::initialize("sprite", 800, 600)?;
//!     engine.add_node(timer::TimerNode::new(10))?;
//!
//!     let tex = engine
//!         .loader()
//!         .load_texture2d("./Core/TestData/IO/AltseedPink256.png")?;
//!
//!     let node = Sprite::new().with_tex(tex).into_node();
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
