use altseed2::prelude::*;

include!("../src/tests/timer.rs");

#[test]
fn tree_add() -> AltseedResult<()> {
    let engine = Engine::initialize("Tree Add", 800, 600)?;
    engine.add_node(timer::TimerNode::new(5))?;

    let t1 = engine
        .loader()
        .load_texture2d("./Core/TestData/IO/AltseedPink.png")?;

    let s1 = {
        let sprite = Sprite::new();
        sprite
            .borrow_mut()
            .set_texture(&t1)
            .set_src(Rect::new(100.0, 100.0, 100.0, 100.0))
            .set_pos(Vector2::new(100.0, 100.0));
        DrawnNode::new(sprite)
    };

    let s2 = {
        let sprite = Sprite::new();
        sprite
            .borrow_mut()
            .set_texture(&t1)
            .set_src(Rect::new(200.0, 200.0, 100.0, 100.0))
            .set_pos(Vector2::new(200.0, 200.0));
        DrawnNode::new(sprite)
    };

    s1.borrow_mut().add_child(s2)?;
    engine.add_node(s1)?;

    let mut count = 0;
    engine.run_with(|e| {
        // if count == 1 {
        // }
        Ok(())
    })?;

    Ok(())
}
