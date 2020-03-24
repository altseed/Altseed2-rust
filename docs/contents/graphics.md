# 描画について（DrawnNode・CameraNode・他）

## DrawnNode
[DrawnNode](../../node/drawn/struct.DrawnNode.html)は描画を行うためのノードです。

### データ構造
概ねこのような構造になっています。

```rust
// 画像描画
struct Sprite { /* 省略 */ }
// 文字列描画
struct Text { /* 省略 */ }
// ポリゴン描画
struct Polygon { /* 省略 */ }

// 描画の種類
enum DrawnKind {
    Sprite(Sprite),
    Text(Text),
    Polygon(Polygon),
}

// 描画の種類と管理用の値を保持
struct DrawnNode {
    // 描画の種類
    kind: DrawnKind,
    // 描画されるかどうか
    is_drawn: bool,
    // 描画順
    z_order: i32,
    // 描画先のカメラの指定
    camera_group: u32,
}
```

### 使い方

以下のような記述が可能です。

```ignore
// 画像を読み込み
let tex1 = engine.loader().load_texture2d("hoge.png")?;

// node: Rc<RefCell<DrawnNode>>
let node = Sprite::new()
    // 画像を指定
    .with_tex(tex1)
    // 描画範囲指定
    .with_src(Rect::new(100.0, 100.0, 100.0, 100.0))
    // 位置指定
    .with_pos(Vector2::new(100.0, 100.0))
    // SpriteからRc<RefCell<DrawnNode>>に変換
    .into_node();

// エンジンにノードを登録
engine.add_node(node.clone())?;
```

[into_node](../../node/drawn/trait.Drawn.html#method.into_node)の代わりに[DrawnNode::new](../../node/drawn/struct.DrawnNode.html#method.new)を利用することも可能です。

### `Rc<RefCell<DrawnNode>>`から`Sprite`に変更を加える

```ignore
// `if let`を使って取り出す。`sprite`は`&mut Sprite`
if let DrawnKind::Sprite(sprite) = node.borrow_mut().kind_mut() {

    let tex2 = engine.loader().load_texture2d("hoge.png")?;
    // 画像を変更
    sprite.set_tex(tex2);
    // 位置を変更
    sprite.pos_mut() = Vector2::new(200.0, 200.0);
}

```

## CameraNode
[CameraNode](../../node/camera/struct.CameraNode.html)はDrawnNodeの描画先を指定するためのノードです。

描画対象のDrawnNode群は、`u32`のカメラグループの値をビットANDすることで指定されます。

例えば以下の時、
```ignore
camera1: CameraNode { group: 0x001 }
camera2: CameraNode { group: 0x010 }

node1: DrawnNode { camera_group: 0x001 }
node2: DrawnNode { camera_group: 0x010 }
node3: DrawnNode { camera_group: 0x011 }
```

`camera1`では`node1`と`node3`が、
`camera2`では`node2`と`node3`が描画対象になります。

### カメラグループの指定
**CameraNode**:  
[CameraNode::get_group](../../node/camera/struct.CameraNode.html#method.get_group),
[CameraNode::set_group](../../node/camera/struct.CameraNode.html#method.set_group)を利用します。

**DrawnNode**:  
[DrawnNode::get_camera_group](../../node/drawn/struct.DrawnNode.html#method.get_camera_group),
[DrawnNode::set_camera_group](../../node/drawn/struct.DrawnNode.html#method.set_camera_group)を利用します。

