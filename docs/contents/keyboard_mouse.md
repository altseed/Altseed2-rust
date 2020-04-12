# キーボード・マウスについて

## キーボード
[`Keyboard`](../../core/struct.Keyboard.html)はキーボード入力を管理します。
[`Engine::keyboard`](../../engine/struct.Engine.html#method.keyboard)を利用してください。

[`Keyboard::get_key_state`](../../core/struct.Keyboard.html#method.get_key_state)を利用して、指定したキーの状態を取得します。

## マウス
[`Mouse`](../../core/struct.Mouse.html)はマウス入力を管理します。
[`Engine::mouse`](../../engine/struct.Engine.html#method.mouse)を利用してください。

### カーソル座標を取得・設定する
[`Mouse::get_position`](../../core/struct.Mouse.html#method.get_position),
[`Mouse::set_position`](../../core/struct.Mouse.html#method.set_position)
を利用します。

### マウスボタン入力を取得する
[`Mouse::get_mouse_button_state`](../../core/struct.Mouse.html#method.get_mouse_button_state)を利用して、指定したボタンの状態を取得します。

### ホイール入力を取得する
[`Mouse::get_wheel`](../../core/struct.Mouse.html#method.get_wheel)を利用して、-1 ~ 1の範囲でホイールの回転量を取得します。

### マウスカーソルモードを取得・設定する
[`Mouse::get_cursor_mode`](../../core/struct.Mouse.html#method.get_cursor_mode),
[`Mouse::set_cursor_mode`](../../core/struct.Mouse.html#method.set_cursor_mode)を利用します。

- [`CursorMode::Normal`](../../core/enum.CursorMode.html#variant.Normal): デフォルト値
- [`CursorMode::Hidden`](../../core/enum.CursorMode.html#variant.Hidden): カーソル非表示の状態
- [`CursorMode::Disable`](../../core/enum.CursorMode.html#variant.Disable): カーソルの入力が無効の状態。カーソルがウィンドウ中央にロックされます。

### カーソル画像を変更する
[`Cursor::create`](../../core/struct.Cursor.html#method.create)を利用して、画像のファイルパスから[`Cursor`](../../core/struct.Cursor.html)を作成します。
それを[`Mouse::set_cursor_image`](../../core/struct.Mouse.html#method.set_cursor_image)に指定します。

**NOTE**: ホットスポットとは、カーソルのクリック判定の出る座標のことです。
座標は画像の中の相対座標で指定してください。 画像はホットスポットが中心となります。