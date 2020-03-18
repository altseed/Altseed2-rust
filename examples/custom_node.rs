use altseed2::*;
use altseed2::prelude::*;

use std::{rc::Rc, cell::RefCell};

// マクロでNodeを宣言(フィールドが自動で追加されます。)
// std::fmt::Debugトレイと、altseedのHasBaseNodeトレイトが自動で実装されます。
define_node! {
    // ここにアトリビュートを記述可能
    // #[hoge(fuga)]
    pub struct CustomNode {
        count: i32
    }
}

impl CustomNode {
    fn new() -> Rc<RefCell<Self>> {
        // マクロで作成(フィールドが自動で初期化されます。)
        create_node!(
            CustomNode {
                count: 0
            }
        )
    }
}

// Nodeトレイトで呼び出される関数を実装
impl Node for CustomNode {
    fn on_added(&mut self, _: &mut Engine) -> AltseedResult<()> {
        println!("On added to parent node");

        // 正常終了
        Ok(())
    }

    // fn on_updating(&mut self, _: &mut Engine) -> AltseedResult<()> {
    //     println!("On updating");

    //     Ok(())
    // }
    
    // 引数でEngineへの参照を受け取る
    fn on_updated(&mut self, engine: &mut Engine) -> AltseedResult<()> {
        if self.count == 60 {
            engine.close();
            return Ok(());
        }
        self.count += 1;
        println!("On updated: {}", self.count);

        // 正常終了
        Ok(())
    }

    fn on_removed(&mut self, _: &mut Engine) -> AltseedResult<()> {
        println!("On removed from parent node");

        // 正常終了
        Ok(())
    }
}

fn main() -> AltseedResult<()> {
    // Altseedを初期化します。 ?演算子を利用してError時に早期終了します。
    let mut engine = Engine::initialize("engine", 800, 600)?;

    let node = CustomNode::new();

    // on_addedは実際にはここで呼び出されず、メインループまで遅延される
    engine.add_node(node)?;
    
    // 所有権を渡してメインループを実行します。
    engine.run()?;
    // engine が dropする際に自動的にAltseedの終了処理が呼ばれます。

    // 正常終了
    Ok(())
}
