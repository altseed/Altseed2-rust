# 描画機能について

C#版と違って、Rust版ではノードツリーを提供していません。  
描画に関する情報を格納する[DrawnComponent](../../component/drawn/struct.DrawnComponent.html)の配列を管理するだけです。

以下のコードは、新しい[DrawnComponent](../../component/drawn/struct.DrawnComponent.html)をエンジンに追加するサンプルです。

```no_run
use altseed2::prelude::*;

fn main() -> AltseedResult<()> {
    // Altseedを初期化します。
    let mut engine = Engine::initialize("sprite", 800, 600)?;

    // 画像を読み込みます。
    let tex = engine
        .loader()
        .load_texture2d("./Core/TestData/IO/AltseedPink.png")?;

    // 画像を描画するSpriteを基にDrawnComponentを作成します。
    let sprite = Sprite::new().with_texture(tex).build();
  
    // DrawnComponentをengineに登録してIDを取得します。
    let id = engine.drawn_storage_mut().add(sprite);

    engine.run()?;

    Ok(())
}
```

画像を表示するサンプル: [Examples/Sprite](../../examples/_01_sprite.rs)

## DrawnID・DrawnStorageについて
[DrawnID](../../component/drawn/struct.DrawnID.html)は`DrawnComponent`にアクセスするための値を持った構造体です。
[DrawnID](../../component/drawn/struct.DrawnID.html)が`Drop`されると、対応した`DrawnComponen`が自動的に削除されます。

[DrawnStorage](../../component/drawn/struct.DrawnStorage.html)は[`DrawnComponent`](../../component/drawn/struct.DrawnComponent.html)を管理するための構造体です。


[DrawnStorage::add](../../component/drawn/struct.DrawnStorage.html#method.add): 新しいDrawnComponentを追加します。
[DrawnStorage::remove](../../component/drawn/struct.DrawnStorage.html#method.remove): 登録されているDrawnComponentを削除します。`DrawnID`の`Drop`による削除と異なり、削除した`DrawnComponent`を取得できます。

[DrawnStorage::with](../../component/drawn/struct.DrawnStorage.html#method.with): `&DrawnComponent`を受け取っって実行する関数を指定します。

[CameraStorage::with](../../component/camera/struct.CameraStorage.html#method.with): `&mut CameraComponent`を受け取っって実行する関数を指定します。

## DrawnComponentについて
[DrawnComponent`](../../component/drawn/struct.DrawnComponent.html)のフィールドは以下です。

- `z_order: (i32)`: 描画順を指定します。
- `camera_group u32`: ビットANDを使って描画対象のカメラを指定します。
- `kind: `[`DrawnKind`](../../component/drawn_kind/enum.DrawnKind.html)):種類ごとに
  [ sSprite](../../component/drawn_kind/struct.Sprite.html),
  [Text](../../component/drawn_kind/struct.Text.html),
  [`Polygon`](../../component/drawn_kind/struct.Polygon.html),
  に分かれています。

- [DrawnComponent::transform_mut](../../component/drawn/struct.DrawnComponent.html#methods.transform_mut.html): 格納しているDrawnKindが `transform` を保持していた時のみ`Transform`への可変参照を返取得します。

## DrawnKindについて
[DrawnKind](../../component/drawn_kind/enum.DrawnKind.html): 以下の描画の種類を格納します。

[Sprite](../../component/drawn_kind/struct.Sprite.html): 画像の描画を表します。

[Text](../../component/drawn_kind/struct.Text.html): 文字列の描画を表します。

[Polygon](../../component/drawn_kind/struct.Polygon.html): ポリゴン単位での描画を表します。

詳しくはそれぞれのリファレンスをご覧ください。

## CameraComponentについて
ID・Storageに関してはDrawnComponentと同じです。

- `group: u32`: ビットANDを使って描画対象の`DawnComponent`を指定します。
