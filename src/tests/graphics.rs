use crate::auto_generated_core_binding::*;
use crate::math::Matrix44;
use crate::structs::Rect;
use crate::structs::*;

#[test]
fn rendered_sprite() {
    assert!(Core::initialize(
        "rendered sprite",
        800,
        600,
        &mut Configuration::new().unwrap()
    ));

    let t1 = Texture2D::load("./Core/TestData/IO/AltseedPink.png").unwrap();
    let t2 = Texture2D::load("./Core/TestData/IO/AltseedPink.jpg").unwrap();

    let mut x1 = RenderedSprite::create().unwrap();
    let mut x2 = RenderedSprite::create().unwrap();

    x1.set_texture(t1.clone())
        .set_src(Rect::new(0.0, 0.0, 128.0, 128.0));

    x2.set_texture(t2)
        .set_transform(Matrix44::translation(200.0, 200.0, 0.0))
        .set_src(Rect::new(128.0, 128.0, 256.0, 256.0));

    let mut core = Core::get_instance().unwrap();
    let mut graphics = Graphics::get_instance().unwrap();
    let mut renderer = Renderer::get_instance().unwrap();

    let mut count = 0;
    while core.do_event() && graphics.do_events() && count < 5 {
        count += 1;

        assert!(graphics.begin_frame());

        renderer.draw_sprite(&mut x1);
        renderer.draw_sprite(&mut x2);

        let mut cmdlist = graphics.get_command_list().unwrap();
        // cmdlist.set_render_target_with_screen();
        renderer.render(&mut cmdlist);
        assert!(graphics.end_frame());
    }

    Core::terminate();
}

#[test]
fn rendered_text() {
    assert!(Core::initialize(
        "rendered text",
        800,
        600,
        &mut Configuration::new().unwrap()
    ));

    let font = Font::load_dynamic_font("./Core/TestData/Font/mplus-1m-regular.ttf", 50).unwrap();
    let imagefont = Font::create_image_font(&mut font.lock().unwrap()).unwrap();
    let tex = Texture2D::load("./Core/TestData/IO/AltseedPink.png").unwrap();
    imagefont
        .lock()
        .unwrap()
        .add_image_glyph::<Texture2D>('〇' as i32, &mut tex.borrow_mut());

    let mut x1 = RenderedText::create().unwrap();
    let mut x2 = RenderedText::create().unwrap();

    x1.set_font(font)
        .set_text("Hello, world! こんにちは".to_owned())
        .set_color(Color::new3(0, 0, 255))
        .set_transform(Matrix44::translation(0.0, 0.0, 0.0));

    x2.set_font(imagefont)
        .set_text("Altseed〇Altseed".to_owned())
        .set_color(Color::new3(0, 0, 0))
        .set_transform(Matrix44::translation(0.0, 150.0, 0.0));

    let mut core = Core::get_instance().unwrap();
    let mut graphics = Graphics::get_instance().unwrap();
    let mut renderer = Renderer::get_instance().unwrap();

    let mut count = 0;
    while core.do_event() && graphics.do_events() && count < 10 {
        count += 1;
        assert!(graphics.begin_frame());

        // cmdlist.set_render_target_with_screen();

        renderer.draw_text(&mut x1);
        renderer.draw_text(&mut x2);

        let mut cmdlist = graphics.get_command_list().unwrap();
        renderer.render(&mut cmdlist);
        assert!(graphics.end_frame());
    }

    Core::terminate();
}

#[test]
fn rendered_camera() {
    assert!(Core::initialize(
        "rendered sprite",
        800,
        600,
        &mut Configuration::new().unwrap()
    ));

    let t1 = Texture2D::load("./Core/TestData/IO/AltseedPink.png").unwrap();
    let t2 = Texture2D::load("./Core/TestData/IO/AltseedPink.jpg").unwrap();

    let mut x1 = RenderedSprite::create().unwrap();
    let mut x2 = RenderedSprite::create().unwrap();

    x1.set_texture(t1.clone())
        .set_src(Rect::new(0.0, 0.0, 128.0, 128.0));

    x2.set_texture(t2)
        .set_transform(Matrix44::translation(200.0, 200.0, 0.0))
        .set_src(Rect::new(128.0, 128.0, 256.0, 256.0));

    let mut camera = RenderedCamera::create().unwrap();
    camera.set_transform(Matrix44::translation(0.0, 0.0, 0.0));

    let mut core = Core::get_instance().unwrap();
    let mut graphics = Graphics::get_instance().unwrap();
    let mut renderer = Renderer::get_instance().unwrap();

    let mut count = 0;
    while core.do_event() && graphics.do_events() && count < 5 {
        count += 1;

        assert!(graphics.begin_frame());

        renderer.set_camera(&mut camera);
        renderer.draw_sprite(&mut x1);
        renderer.draw_sprite(&mut x2);
        // renderer.reset_camera();

        let mut cmdlist = graphics.get_command_list().unwrap();
        // cmdlist.set_render_target_with_screen();
        renderer.render(&mut cmdlist);
        assert!(graphics.end_frame());
    }

    Core::terminate();
}
