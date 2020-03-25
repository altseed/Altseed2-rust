// Auto-generated. Do not modify.
// このファイルは自動生成されたものです。変更しないでください。

//! ```
//! use altseed2::prelude::*;
//!
//! fn main() -> AltseedResult<()> {
//!     // Altseedを初期化します。 ?演算子を利用してError時に早期終了します。
//!     let mut engine = Engine::initialize("sound", 800, 600)?;
//!
//!     // 音ファイルを読み込みます。
//!     // 効果音の場合は第2引数を true に設定して事前にファイルを解凍することが推奨されている。
//!     let se = engine
//!         .loader()
//!         .load_sound("./Core/TestData/Sound/se1.wav", true)?;
//!
//!     let id = engine.sound().play(&mut se.lock().unwrap())?;
//!
//!     // 所有権を渡してメインループを実行します。\
//!     // 毎フレーム実行される関数を指定できます。引数としてエンジンの可変参照を受け取ることができます。
//!     engine.run_with(|e| {
//!         // 音の再生が終了しているか調べる。
//!         if !e.sound().is_playing(id) {
//!             // メインループを終了する
//!             e.close();
//!         }
//!
//!         // 正常終了
//!         Ok(())
//!     })?;
//!     // engineがdropする際に自動的にAltseedの終了処理が呼ばれます。
//!
//!     // 正常終了
//!     Ok(())
//! }
