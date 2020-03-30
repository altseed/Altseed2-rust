use crate::component::drawn::DRAWN_STORAGE;
use crate::prelude::*;

#[test]
fn draw_sprite() -> AltseedResult<()> {
    let mut engine = Engine::initialize("sprite", 800, 600)?;
    println!("Engine initialized");
    DRAWN_STORAGE.with(|s| println!("{:?}", s.borrow()));

    let tex = engine
        .loader()
        .load_texture2d("./Core/TestData/IO/AltseedPink.png")?;

    let size: Vector2<f32> = tex.lock().unwrap().get_size().into();

    let sprite = Sprite::new()
        .with_texture(tex)
        .with_center(size * 0.5)
        .build();

    let mut sprite_id = Some(engine.drawn_storage_mut().add(sprite));
    println!("sprite_id: {:?}", &sprite_id);

    let mut count = 0;
    engine.run_with(|e| {
        println!("{}", count);
        if count == 60 {
            println!("remove");
            let c = e.drawn_storage_mut().remove(sprite_id.take().unwrap());
            println!("{:?}\n", c);
        }

        let fps = e.get_current_fps();

        match &sprite_id {
            Some(id) => e.drawn_storage_mut().with_mut(id, |d| {
                let trans = d.transform_mut().unwrap();
                *trans.angle_mut() += 0.1 * fps / 60.0;
            }),
            None => {
                println!("Not Found!");
                assert_eq!(e.drawn_storage().len(), 0);
                e.close();
            }
        };

        count += 1;

        Ok(())
    })?;

    println!("finish");

    Ok(())
}

#[test]
fn drawn_z_order() -> AltseedResult<()> {
    DRAWN_STORAGE.with(|s| println!("{:?}", s.borrow()));
    let mut engine = Engine::initialize("sprite", 800, 600)?;
    println!("Engine initialized");
    DRAWN_STORAGE.with(|s| println!("{:?}", s.borrow()));

    let tex = engine
        .loader()
        .load_texture2d("./Core/TestData/IO/AltseedPink.png")?;

    let size: Vector2<f32> = tex.lock().unwrap().get_size().into();

    let sprite = Sprite::new().with_texture(tex.clone()).build();
    let id1 = engine.drawn_storage_mut().add(sprite);
    println!("id1: {:?}", &id1);

    DRAWN_STORAGE.with(|s| println!("{:?}", s.borrow()));

    let sprite = Sprite::new().with_texture(tex).with_pos(size * 0.2).build();
    let id2 = engine.drawn_storage_mut().add(sprite);
    println!("id2: {:?}", &id2);

    DRAWN_STORAGE.with(|s| println!("{:?}", s.borrow()));

    let mut count = 0;
    engine.run_with(|e| {
        println!("{}", count);
        if count == 60 {
            e.close();
        }

        if count % 15 == 0 {
            e.drawn_storage_mut().with_mut(&id1, |d| {
                *d.z_order_mut() = (count / 15) % 2;
            });

            e.drawn_storage_mut().with_mut(&id2, |d| {
                *d.z_order_mut() = (count / 15 + 1) % 2;
            });
        }

        count += 1;

        Ok(())
    })?;

    Ok(())
}
