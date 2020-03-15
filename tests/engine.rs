use altseed2::prelude::*;

#[test]
fn engine() -> AltseedResult<()> {
    let mut engine = Engine::initialize("engine", 800, 600)?;
    let mut count = 0;
    println!(
        "window title: {:?}",
        engine.core().borrow_mut().get_window_title()
    );
    while count < 20 && engine.do_events() {
        count += 1;
        engine.update()?;
    }

    Ok(())
}

use std::{cell::RefCell, rc::Rc};

#[test]
fn node() -> AltseedResult<()> {
    let mut engine = Engine::initialize("node", 800, 600)?;
    let node = Rc::new(RefCell::new(NodeBase::default()));
    engine.add_node(node).unwrap();

    let mut count = 0;
    while count < 20 && engine.do_events() {
        count += 1;
        engine.update()?;
    }

    Ok(())
}

#[test]
fn sprite() -> AltseedResult<()> {
    let mut engine = Engine::initialize("sprite", 800, 600)?;

    let texture = engine
        .loader()
        .load_texture2d("./Core/TestData/IO/AltseedPink.png")?;

    let node = {
        let sprite = Sprite::new();

        sprite
            .borrow_mut()
            .set_texture(&texture)
            .set_src(Rect::new(0.0, 0.0, 128.0, 128.0));

        DrawnNode::new(sprite)
    };

    engine.add_node(node)?;

    let mut count = 0;
    while count < 20 && engine.do_events() {
        count += 1;
        engine.update()?;
    }

    Ok(())
}
