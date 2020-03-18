// Auto-generated. Do not modify.
// このファイルは自動生成されたものです。変更しないでください。
//! カスタムノードを作成して使用するサンプル
//! ```no_run
//! use altseed2::prelude::*;
//! use altseed2::*;
//! use std::{cell::RefCell, rc::Rc};
//!
//! // マクロでNodeを宣言(BaseNode構造体のフィールドが自動で追加されます。)
//! // std::fmt::DebugトレイとaltseedのHasBaseNodeトレイトが自動で実装されます。
//! define_node! {
//!     // ここにアトリビュートを記述可能
//!     // #[hoge(fuga)]
//!     pub struct TimerNode {
//!         count: i32,
//!         limit: i32,
//!     }
//! }
//!
//! impl TimerNode {
//!     pub fn new(limit: i32) -> Rc<RefCell<Self>> {
//!         // マクロで作成(フィールドが自動で初期化されます。)
//!         Rc::new(RefCell::new(create_node!(TimerNode {
//!             count: 0,
//!             limit: limit
//!         })))
//!     }
//! }
//!
//! // Nodeトレイトで呼び出される関数を実装
//! impl Node for TimerNode {
//!     fn on_added(&mut self, _: &mut Engine) -> AltseedResult<()> {
//!         println!("Started");
//!
//!         // 正常終了
//!         Ok(())
//!     }
//!
//!     // 引数でEngineへの参照を受け取る
//!     fn on_updated(&mut self, engine: &mut Engine) -> AltseedResult<()> {
//!         println!("count: {}", self.count);
//!
//!         if self.count == self.limit {
//!             engine.close();
//!             println!("Finished");
//!             return Ok(());
//!         }
//!
//!         self.count += 1;
//!
//!         // 正常終了
//!         Ok(())
//!     }
//! }
//!
//! fn main() -> AltseedResult<()> {
//!     let engine = Engine::initialize("Custom Node", 800, 600)?;
//!
//!     engine.add_node(TimerNode::new(60))?;
//!
//!     engine.run()?;
//!
//!     Ok(())
//! }
//!
//! ```
//!
//! ## Run this example
//! ```shell
//! cargo run --example custom_node
//! ```
