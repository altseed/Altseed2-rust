// Auto-generated. Do not modify.
// このファイルは自動生成されたものです。変更しないでください。

//! # 画像を表示するサンプル
//! ```no_run
//! use altseed2::prelude::*;
//!
//! // 返り値に`AltseedResult<()>`を指定して?演算子を使用可能にします。
//! fn main() -> AltseedResult<()> {
//!     // Altseedを初期化します。 ?演算子を利用してError時に早期終了します。
//!     let mut engine = Engine::initialize("sprite", 800, 600)?;
//!
//!     // 画像を読み込みます。
//!     let tex = engine
//!         .loader()
//!         .load_texture2d("./Core/TestData/IO/AltseedPink.png")?;
//!
//!     // 画像を格納するDrawnComponentを作成します。
//!     let sprite = Sprite::new().with_texture(tex).build();
//!     // DrawnComponentをengineに登録してIDを取得します。
//!     let id = engine.drawn_storage_mut().add(sprite);
//!
//!     let mut count = 0;
//!     // 所有権を渡してメインループを実行します。
//!     // 毎フレーム実行される関数を指定できます。
//!     engine.run_with(|e| {
//!         // 取得したidを使って、登録したDrawnComponentにアクセスします。
//!         e.drawn_storage_mut().with_mut(&id, |d| {
//!             // 位置を更新します。
//!             *d.transform_mut().unwrap().pos_mut() += Vector2::new(2.0, 0.0);
//!         });
//!
//!         if count == 60 {
//!             // Altseedを終了するフラグを立てます。
//!             e.close();
//!         }
//!         count += 1;
//!         Ok(())
//!     })?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Run this example
//! ```shell
//! cargo run --example sprite
//! ```
