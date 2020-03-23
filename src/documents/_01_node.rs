// Auto-generated. Do not modify.
// このファイルは自動生成されたものです。変更しないでください。

//! # ノード（オブジェクトシステム）について
//!
//! Altseed2では、ゲームに登場する**キャラクター等のオブジェクト**、及び**オブジェクトが持つ機能**等をノードとして管理します。
//!
//! 例えばゲームに画像を表示する際には、[Sprite](../../node/sprite/struct.Sprite.html)を保持した
//! [DrawnNode](../../node/drawn/struct.DrawnNode.html)をエンジンに追加します。
//! ([examles/sprite](../../examples/_01_sprite/index.html))
//!
//! 描画ノードについては[Documents/Graphics](../graphics/index.html)をご覧ください。
//!
//! Rust版Altseed2では[Node](../../node/trait.Node.html)トレイトを実装することでノードの処理を記述します。
//!
//! [Node](../../node/trait.Node.html)トレイトが継承する[HasBaseNode](../../node/trait.HasBaseNode.html)トレイトが、
//! 子ノードを追加する関数や自身を親ノードから削除する関数などノードの基本的な機能を持っています。
//!
//! ## ノードの追加削除について
//! **注意**: [Node](../../node/trait.Node.html)の追加削除は呼び出し時に行われるのではなく、次の更新時まで遅延されます。
//!
//! ### エンジンにノードを追加する
//! [Engine::add_node](../../engine/struct.Engine.html#method.add_node)を利用します。
//!
//! ### ノードに子ノードを追加する。
//! [HasBaseNode::add_child](../../node/trait.HasBaseNode.html#method.add_child)を利用します。
//!
//! ### ノードを親ノードから削除する。
//! [HasBaseNode::remove](../../node/trait.HasBaseNode.html#method.remove)を利用して、自身を所属する親ノードから削除することができます。
//!
//! ### 現在のノードの登録状態を取得する
//! [HasBaseNode::state](../../node/trait.HasBaseNode.html#method.state)を利用します。
//!
//! ## 親子構造
//! ノードは親子関係の階層構造を持ちます。親ノードが更新された後、子のノードが再帰的に実行されます。
//!
//! また、DrawnNodeのTransformは親から子に伝播されます。
//! DrawnNodeでない場合は変更を与えずに伝播します。
//!
//! ## 独自のノードを利用する。
//! [define_node!](../../macro.define_node.html)マクロを利用して独自のノードを作成できます。([examles/custom_node](../../examples/_06_custom_node/index.html))
//!
//!
//! ```no_run
//! use altseed2::prelude::*;
//! use altseed2::*;
//!
//! use std::{rc::Rc, cell::RefCell};
//!
//! define_node! {
//!     pub struct CustomNode {
//!         // マクロによってBaseNodeが埋め込まれる。
//!         foo: i32,
//!         bar: f32,
//!     }
//!
//!     // マクロによってHasBaseNodeトレイトが実装される。
//! }
//!
//! // Nodeトレイトは明示的に実装する必要があります。
//! impl Node for CustomNode {
//!     fn on_updated(&mut self, engine: &mut Engine) -> AltseedResult<()> {
//!         // 処理を記述する
//!         Ok(())
//!     }
//! }
//!
//! impl CustomNode {
//!     pub fn new() -> Rc<RefCell<Self>> {
//!         Rc::new(RefCell::new(
//!             create_node!(CustomNode {
//!                 // マクロによってBaseNodeが埋め込まれる
//!                 foo: 0,
//!                 bar: 0.0,
//!             })
//!         ))
//!     }
//! }
//! ```
