# 描画機能について

C#版と違って、Rust版ではノードツリーを提供していません。  
描画に関する情報を格納する[`DrawnComponent`](../../component/drawn/struct.DrawnComponent.html)の配列を管理するだけです。

以下のコードは、新しい[`DrawnComponent`](../../component/drawn/struct.DrawnComponent.html)をエンジンに追加するサンプルです。

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
    let _id = engine.drawn_storage_mut().add(sprite);

    engine.run()?;

    Ok(())
}
```

画像を表示するサンプル: [Examples/Sprite](../../examples/_01_sprite.rs)

## DrawnID・DrawnStorageについて
[`DrawnID`](../../component/drawn/struct.DrawnID.html)は`Engine`経由で`DrawnComponent`にアクセスするための構造体です。
`DrawnComponent`への所有権を表しており、このIDが`Drop`すると対応した`DrawnComponen`が自動的に削除されます。

[`DrawnStorage`](../../component/drawn/struct.DrawnStorage.html)は[`DrawnComponent`](../../component/drawn/struct.DrawnComponent.html)を管理するための構造体です。

[`DrawnStorage::add`](../../component/drawn/struct.DrawnStorage.html#method.add): 新しい`DrawnComponent`を追加します。
[`DrawnStorage::remove`](../../component/drawn/struct.DrawnStorage.html#method.remove): 登録されている`DrawnComponent`を削除します。`DrawnID`の`Drop`による削除と異なり、削除した`DrawnComponent`を取得できます。

[`DrawnStorage::with`](../../component/drawn/struct.DrawnStorage.html#method.with): `&DrawnComponent`を受け取る関数を実行します。

[`CameraStorage::with`](../../component/camera/struct.CameraStorage.html#method.with): `&mut CameraComponent`を受け取る関数を実行します。

## DrawnComponentについて
[`DrawnComponent`](../../component/drawn/struct.DrawnComponent.html)は以下のものから構成されます。

- `z_order`: `i32`で描画順を表します。
- `camera_group`: `u32`で描画対象のカメラグループを指定します。ビットANDを行うため、複数のカメラへの描画も指定できます。
- `kind`: [`DrawnKind`](../../component/drawn_kind/enum.DrawnKind.html))の種類ごとに
  [`Sprite`](../../component/drawn_kind/struct.Sprite.html),
  [`Text`](../../component/drawn_kind/struct.Text.html),
  [`Polygon`](../../component/drawn_kind/struct.Polygon.html),
  に分かれています。

- [`DrawnComponent::transform_mut`](../../component/drawn/struct.DrawnComponent.html#methods.transform_mut.html)
  , [`DrawnComponent::transform`](../../component/drawn/struct.DrawnComponent.html#methods.transform.html)
  : 格納しているDrawnKindが[`Transform`](../../math/transform/struct.Transform.html)を保持していた時のみ参照を取得します。

## DrawnKindについて
[`DrawnKind`](../../component/drawn_kind/enum.DrawnKind.html)は以下の描画の種類を格納します。

- [`Sprite`](../../component/drawn_kind/struct.Sprite.html): 画像の描画を表します。

- [`Text`](../../component/drawn_kind/struct.Text.html): 文字列の描画を表します。

- [`Polygon`](../../component/drawn_kind/struct.Polygon.html): ポリゴン単位での描画を表します。

詳しくはそれぞれのリファレンスをご覧ください。

## CameraComponentについて
[`CameraID`](../../components/camera/struct.CameraID.html)・
[`CameraStorage`](../../components/camera/struct.CameraStorage.html)
は`DrawnComponent`のものと同じように扱えます。

- `group`: `u32`でカメラグループを指定します。
