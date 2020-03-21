use altseed2::math::Matrix44;
use altseed2::prelude::*;

include!("../src/tests/timer.rs");

#[test]
fn camera_no_rt() -> AltseedResult<()> {
    let engine = Engine::initialize("Camera with no target", 800, 600)?;
    engine.add_node(timer::TimerNode::new(60))?;

    let tex = engine
        .loader()
        .load_texture2d("./Core/TestData/IO/AltseedPink.png")?;
    // let tex_size: Vector2<f32> = tex.borrow_mut().get_size().into();
    {
        let node = Sprite::new().with_tex(tex).build();
        node.borrow_mut().set_camera_group(1);
        // node.borrow_mut().set_center(tex_size / 2.0);
        engine.add_node(node)?;
    }

    {
        let camera = CameraNode::new();
        camera
            .borrow_mut()
            .set_transform(Matrix44::translation(-400.0, -100.0, 0.0))
            .set_group(1);
        engine.add_node(camera)?;
    }

    engine.run()?;

    Ok(())
}
