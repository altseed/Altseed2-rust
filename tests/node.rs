use altseed2::prelude::*;
use altseed2::*;
use std::{cell::RefCell, rc::Rc};

include!("../src/tests/timer.rs");

#[test]
fn tree_add() -> AltseedResult<()> {
    let engine = Engine::initialize("Tree Add", 800, 600)?;
    engine.add_node(timer::TimerNode::new(5))?;

    let t1 = engine
        .loader()
        .load_texture2d("./Core/TestData/IO/AltseedPink.png")?;

    let s1 = Sprite::new()
        .with_tex(t1.clone())
        .with_src(Rect::new(100.0, 100.0, 100.0, 100.0))
        .with_pos(Vector2::new(100.0, 100.0))
        .into_node();

    let s2 = Sprite::new()
        .with_tex(t1)
        .with_src(Rect::new(200.0, 200.0, 100.0, 100.0))
        .with_pos(Vector2::new(200.0, 200.0))
        .into_node();

    s1.borrow_mut().add_child(s2)?;
    engine.add_node(s1)?;

    engine.run()?;

    Ok(())
}

#[test]
fn node_remove() -> AltseedResult<()> {
    let engine = Engine::initialize("Node Remove", 800, 600)?;

    let t1 = engine
        .loader()
        .load_texture2d("./Core/TestData/IO/AltseedPink.png")?;

    let s1 = Sprite::new()
        .with_tex(t1.clone())
        .with_src(Rect::new(100.0, 100.0, 100.0, 100.0))
        .with_pos(Vector2::new(100.0, 100.0))
        .into_node();

    let s2 = Sprite::new()
        .with_tex(t1)
        .with_src(Rect::new(200.0, 200.0, 100.0, 100.0))
        .with_pos(Vector2::new(100.0, 100.0))
        .into_node();

    s1.borrow_mut().add_child(s2.clone())?;
    engine.add_node(s1.clone())?;

    let mut count = 0;
    engine.run_with(|e| {
        match count {
            20 => {
                s1.borrow_mut().remove()?;
                s2.borrow_mut().remove()?;
            }
            40 => {
                println!("adding");
                e.add_node(s2.clone())?;
                println!("added");
            }
            60 => e.close(),
            _ => (),
        }
        count += 1;
        Ok(())
    })?;

    Ok(())
}

define_node! {
    pub struct CustomNode {
        count: i32,
    }
}

impl Node for CustomNode {
    fn on_added(&mut self, _: &mut Engine) -> AltseedResult<()> {
        self.count += 1;
        println!("On Added");
        Ok(())
    }

    fn on_updated(&mut self, _: &mut Engine) -> AltseedResult<()> {
        self.count += 1;
        println!("On Updated");
        Ok(())
    }

    fn on_removed(&mut self, _: &mut Engine) -> AltseedResult<()> {
        self.count += 1;
        println!("On Removed");
        Ok(())
    }
}

impl CustomNode {
    fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(create_node!(CustomNode { count: 0 })))
    }

    fn count(&self) -> i32 {
        self.count
    }
}

#[test]
fn node_custom() -> AltseedResult<()> {
    let engine = Engine::initialize("Node Custom", 800, 600)?;

    let node = CustomNode::new();
    engine.add_node(node.clone())?;

    let mut count = 0;
    engine.run_with(|e| {
        // 0 => on_added
        // 1 => on_updating, on_updated
        // 2 => on_removed
        println!("count: {}", count);
        if count == 2 {
            node.borrow_mut().remove()?;
            e.close();
        }
        count += 1;
        Ok(())
    })?;

    assert_eq!(node.borrow().count(), 3);

    Ok(())
}
