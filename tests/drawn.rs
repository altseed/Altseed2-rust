use altseed2::prelude::*;

#[test]
fn draw_sprite() -> AltseedResult<()> {
    let mut engine = Engine::initialize("sprite", 800, 600)?;

    let tex = engine
        .loader()
        .load_texture2d("./Core/TestData/IO/AltseedPink.png")?;

    let size: Vector2<f32> = tex.borrow_mut().get_size().into();

    let sprite = Sprite::new().with_texture(tex).with_center(size * 0.5).build();

    let sprite_id = engine.drawn_storage_mut().push(sprite);

    let mut count = 0;
    let engine = engine.run_with(|e| {
        println!("{}", count);
        if count == 60 {
            e.drawn_storage_mut().remove(sprite_id);
        }

        let fps = e.get_current_fps();

        if let Some(d) = e.drawn_storage_mut().get_mut(sprite_id) {
            if let DrawnKind::Sprite(sprite) = d.kind_mut() {
                *sprite.angle_mut() += 0.1 * fps / 60.0;
            }
        } else {
            println!("Not Found!");
            e.close();
        }

        count += 1;

        Ok(())
    })?;

    println!("{:?}", engine.drawn_storage());

    Ok(())
}

#[test]
fn drawn_z_order() -> AltseedResult<()> {
    let mut engine = Engine::initialize("sprite", 800, 600)?;

    let tex = engine
        .loader()
        .load_texture2d("./Core/TestData/IO/AltseedPink.png")?;

    let size: Vector2<f32> = tex.borrow_mut().get_size().into();

    let id1 = engine
        .drawn_storage_mut()
        .push(Sprite::new().with_texture(tex.clone()).build());

    println!("{:?}", engine.drawn_storage());

    let id2 = engine
        .drawn_storage_mut()
        .push(Sprite::new().with_texture(tex).with_pos(size * 0.2).build());

    println!("{:?}", engine.drawn_storage());

    let mut count = 0;
    engine.run_with(|e| {
        println!("{}", count);
        if count == 60 {
            e.close();
        }

        if count % 15 == 0 {
            {
                let d = e.drawn_storage_mut().get_mut(id1).unwrap();
                *d.z_order_mut() = (count / 15) % 2;
            }

            {
                let d = e.drawn_storage_mut().get_mut(id2).unwrap();
                *d.z_order_mut() = (count / 15 + 1) % 2;
            }
        }

        count += 1;

        Ok(())
    })?;

    Ok(())
}
