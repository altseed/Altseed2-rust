// Auto-generated. Do not modify.
// このファイルは自動生成されたものです。変更しないでください。

//! ```
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
//!     let node = {
//!         let node = Sprite::new().with_tex(tex).build();
//!         node.borrow_mut().set_center(tex_size * 0.5);
//!         node
//!     };
//!
//!     engine.add_node(node.clone())?;
//!
//!     let is_loaded = Rc::new(RefCell::new(false));
//!
//!     engine.run_task({
//!         let root = engine.root_node().clone();
//!         let loader = engine.loader().clone();
//!         let is_loaded = is_loaded.clone();
//!
//!         async move {
//!             println!("Started async block ({:?})", thread::current().id());
//!
//!             println!("Started load file");
//!             let font = POOL
//!                 .spawn_with_handle(async move {
//!                     println!("Started thread pool({:?})", thread::current().id());
//!                     loader.load_dynamic_font("./Core/TestData/Font/mplus-1m-regular.ttf", 50)
//!                 })?
//!                 .await?;
//!             println!("finished load file ({:?})", thread::current().id());
//!
//!             let node = {
//!                 Text::new()
//!                     .with_font(font)
//!                     .with_text("読み込み完了")
//!                     .build()
//!             };
//!
//!             root.borrow_mut().add_child(node)?;
//!             println!("Added Text Node ({:?})", thread::current().id());
//!
//!             *is_loaded.borrow_mut() = true;
//!             println!("Set is_loaded ({:?})", thread::current().id());
//!
//!             Ok(())
//!         }
//!     });
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