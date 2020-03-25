// Auto-generated. Do not modify.
// このファイルは自動生成されたものです。変更しないでください。

//! マルチスレッドでファイルを読み込むサンプル
//! ```no_run
//! #[macro_use]
//! extern crate lazy_static;
//! extern crate futures;
//!
//! use std::{cell::RefCell, f32::consts::PI, rc::Rc, thread};
//!
//! use futures::{executor::ThreadPool, task::SpawnExt};
//!
//! use altseed2::prelude::*;
//!
//! fn main() -> AltseedResult<()> {
//!     lazy_static! {
//!         static ref POOL: ThreadPool = ThreadPool::new().unwrap();
//!     }
//!
//!     let mut engine = Engine::initialize("load async", 800, 600)?;
//!
//!     let tex = engine
//!         .loader()
//!         .load_texture2d("./Core/TestData/IO/AltseedPink.png")?;
//!     let tex_size: Vector2<f32> = tex.borrow_mut().get_size().into();
//!
//!     let node = Sprite::new()
//!         .with_tex(tex)
//!         .with_center(tex_size * 0.5)
//!         .into_node();
//!
//!     engine.add_node(node.clone())?;
//!
//!     let is_loaded = Rc::new(RefCell::new(false));
//!
//!     {
//!         // asyncブロックにmoveするためにcloneする
//!         let loader = engine.loader().clone();
//!         let is_loaded = is_loaded.clone();
//!
//!         // 非同期処理を生成する
//!         engine.spawn_task(async move {
//!             println!("Started async block ({:?})", thread::current().id());
//!
//!             println!("Started load file");
//!
//!             // futuresのThreadPoolを使って別スレッドでフォントファイルの読み込みを行う。
//!             let font = POOL
//!                 .spawn_with_handle(async move {
//!                     println!("Started thread pool({:?})", thread::current().id());
//!                     loader.load_dynamic_font("./Core/TestData/Font/mplus-1m-regular.ttf", 50)
//!                 })?
//!                 .await?;
//!             println!("finished load file ({:?})", thread::current().id());
//!
//!             // Rc<RefCell<bool>>のフラグを書き換える。
//!             *is_loaded.borrow_mut() = true;
//!             println!("Set is_loaded ({:?})", thread::current().id());
//!
//!             // 継続でエンジンにアクセスして処理を行う。
//!             Cont::then(|e: &mut Engine| {
//!                 e.add_node(
//!                     Text::new()
//!                         .with_font(font)
//!                         .with_text("読み込み完了")
//!                         .into_node(),
//!                 )?;
//!                 println!("Added Text Node ({:?})", thread::current().id());
//!
//!                 Ok(())
//!             })
//!         });
//!     }
//!
//!     let mut count = 0;
//!
//!     engine.run_with(|e| {
//!         {
//!             let a = node.borrow().get_angle();
//!             node.borrow_mut().set_angle(a + PI * 0.01);
//!             println!("angle: {}", a);
//!         }
//!
//!         if count == 0 {
//!             if *is_loaded.borrow() {
//!                 count = 1;
//!             }
//!         } else {
//!             count += 1;
//!             if count > 20 {
//!                 e.close();
//!             }
//!         }
//!
//!         Ok(())
//!     })?;
//!     // engineがdropする際に自動的にAltseedの終了処理が呼ばれます。
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Run this example
//! ```shell
//! cargo run --example load_async
//! ```
