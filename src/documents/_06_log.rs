// Auto-generated. Do not modify.
// このファイルは自動生成されたものです。変更しないでください。

//! # ログ出力について
//!
//! [`Log`](../../core/struct.Log.html)はコンソールやファイルにログを出力する機能です。
//! [`Engine::log`](../../engine/struct.Engine.html#method.log)を利用してください。
//!
//! Altseedの内部で発生したエラーなどはこの機能でコンソールやファイルへ出力されます。
//!
//! 以下のように記述して使用します。
//!
//! ```ignore
//! engine.log().debug("Nyan");
//! ```
//!
//! [`LogLevel`](../../core/enum.LogLevel.html)に応じたメソッドが用意されています。
//!
//! ## 出力される最低レベルを設定する
//! [`Log::set_level`](../../core/struct.Log.html#method.set_level)を利用して、[`LogCategory`](../../core/enum.LogCategory.html)別にログを出力時の最低レベルを指定することができます。
//!
//! ## ログの有効・無効化
//! [`Engine::initialize_with`](../../engine/struct.Engine.html#method.initialize_with)で渡す[`Config`](../../engine/struct.Config.html)を指定します。
//!
//! - [`log_console`](../../engine/struct.Config.html#structfield.log_console): コンソールへのログ出力を有効にするかどうかを指定します。
//! - [`log_filename`](../../engine/struct.Config.html#structfield.log_filename): ログ出力する際のファイル名を指定します。`None`を指定するとファイル出力が無効になります。
