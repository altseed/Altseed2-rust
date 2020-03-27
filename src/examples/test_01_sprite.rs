// Auto-generated. Do not modify.
// このファイルは自動生成されたものです。変更しないでください。

//! ```
//! use altseed2::prelude::*;
//!
//! fn main() -> AltseedResult<()> {
//!     let mut engine = Engine::initialize("sprite", 800, 600)?;
//!
//!     let tex = engine
//!         .loader()
//!         .load_texture2d("./Core/TestData/IO/AltseedPink.png")?;
//!
//!     let sprite = Sprite::new().with_texture(tex).build();
//!     let _id = engine.drawn_storage_mut().add(sprite);
//!
//!     let mut count = 0;
//!     engine.run_with(|e| {
//!         if count == 60 {
//!             e.close();
//!         }
//!         count += 1;
//!         Ok(())
//!     })?;
//!
//!     Ok(())
//! }
