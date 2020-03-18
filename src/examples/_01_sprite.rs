// Auto-generated. Do not modify.
// このファイルは自動生成されたものです。変更しないでください。
//! # 画像を表示するサンプル
//! ```no_run
//! use altseed2::prelude::*;
//! 
//! fn main() -> AltseedResult<()> {
//!     // Altseedを初期化します。
//!     let engine = Engine::initialize("sprite", 800, 600)?;
//! 
//!     let tex = engine
//!         .loader()
//!         .load_texture2d("./Core/TestData/IO/AltseedPink256.png")?;
//! 
//!     let node = {
//!         let sprite = Sprite::new();
//!         sprite.borrow_mut().set_texture(&tex);
//!         DrawnNode::new(sprite)
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
//! 
//! ```
//! 
//! ## Run this example
//! ```shell
//! cargo run --example sprite
//! ```

