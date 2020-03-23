// Auto-generated. Do not modify.
// このファイルは自動生成されたものです。変更しないでください。

//! # ジョイスティックについて
//!
//! [Joystick](../../core/struct.Joystick.html)はジョイスティック入出力を管理します。
//! [Engine::joystick](../../engine/struct.Engine.html#method.joystick)を利用してください。
//!
//! ## 接続されているか確認する
//! [Joystick::is_present](../../core/struct.Joystick.html#method.is_present)を利用します。
//!
//! ## 種類を取得する
//! [Joystick::get_joystick_type](../../core/struct.Joystick.html#method.get_joystick_type)を利用します。
//!
//! 現在は以下の種類に対応しています。
//! - [JoystickType::PS4](../../core/enum.JoystickType.html#variant.PS4)
//! - [JoystickType::XBOX360](../../core/enum.JoystickType.html#variant.XBOX360)
//! - [JoystickType::JoyconL](../../core/enum.JoystickType.html#variant.JoyconL)
//! - [JoystickType::JoyconR](../../core/enum.JoystickType.html#variant.JoyconR)
//!
//! それ以外のコントローラーでは[JoystickType::Unknown](../../core/enum.JoystickType.html#variant.Unknown)が返ります。
//!
//! ## 名前を取得する
//! [Joystick::get_joystick_name](../../core/struct.Joystick.html#method.get_joystick_name)を利用します。
//!
//! ## ボタン入力を取得する
//! - [Joystick::get_button_state_by_type](../../core/struct.Joystick.html#method.get_button_state_by_type)
//! : ボタンの種類([JoystickButtonType](../../core/enum.JoystickButtonType.html))から入力を取得します。
//! - [Joystick::get_button_state_by_index](../../core/struct.Joystick.html#method.get_button_state_by_index)
//! : ボタンの番号から入力を取得します。
//!
//! 第一引数に取得したいジョイスティックコントローラーのインデックスを指定します。
//!
//! **NOTE**:
//! ジョイスティックコントローラーのボタンとインデックスの対応はコントローラーごとに異なります。
//! 特定のボタンを取得したい場合は
//! [Joystick::get_button_state_by_index](../../core/struct.Joystick.html#method.get_button_state_by_index)
//! を使うことをおすすめします。
//!
//! ## スティック入力を取得する
//! - [Joystick::get_axis_state_by_type](../../core/struct.Joystick.html#method.get_axis_state_by_type)
//! : スティックの種類([JoystickAxisType](../../core/enum.JoystickAxisType.html))から入力を取得します。
//! - [Joystick::get_axis_state_by_index](../../core/struct.Joystick.html#method.get_axis_state_by_index)
//! : スティックの番号から入力を取得します。
//!
//! ## 振動させる
//! [Joystick::vibrate](../../core/struct.Joystick.html#method.vibrate)を利用します。
//! 第2引数で周波数`(40.0 ~ 1252.0)`、第3引数で振幅`(0.0 ~ 1.0)`を指定できます。
//!
//! 一度このメソッドを実行すると、5秒程度コントローラーが振動します。
//! 振動をキャンセルしたい場合は、振幅に`0`を指定してこのメソッドを実行してください。
//!
//! **CAUTION**:
//! この範囲を超えた場合、範囲内の最も近い値が利用されます。
