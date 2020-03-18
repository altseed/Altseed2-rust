
// Auto-generated. Do not modify.
// このファイルは自動生成されたものです。変更しないでください。

//! # ウィンドウを表示するサンプル
//!
//! ```

//! 
//! use altseed2::prelude::*;
//! 
//! fn main() -> AltseedResult<()> {
//!     // Altseedを初期化します。 ?演算子を利用してError時に早期終了します。
//!     let engine = Engine::initialize("engine", 800, 600)?;
//! 
//!     // ここで画像などのデータを読み込んだりノードツリーを作成したりすることができます。
//! 
//!     // 所有権を渡してメインループを実行します。
//!     engine.run()?;
//!     // engine が dropする際に自動的にAltseedの終了処理が呼ばれます。
//! 
//!     // 正常終了
//!     Ok(())
//! }
//! 

//! ```
//!
//! ## Run this example
//! ```shell
//! cargo run --example hello_world
//! ```

