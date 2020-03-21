use altseed2::prelude::*;

include!("../src/tests/timer.rs");

#[test]
fn set_texture() -> AltseedResult<()> {
    let engine = Engine::initialize("set texture", 800, 600)?;
    engine.add_node(timer::TimerNode::new(20))?;

    let tex1 = engine
        .loader()
        .load_texture2d("./Core/TestData/IO/AltseedPink256.png")?;

    let tex2 = engine
        .loader()
        .load_texture2d("./Core/TestData/IO/AltseedPink.jpg")?;

    let node = Sprite::new().with_tex(tex1.clone()).build();

    engine.add_node(node.clone())?;

    let mut count = 0;
    engine.run_with(|_| {
        if count == 10 {
            if let DrawnKind::Sprite(x) = node.borrow_mut().kind_mut() {
                x.set_tex(tex2.clone());
            }
        }
        count += 1;
        Ok(())
    })?;

    Ok(())
}
