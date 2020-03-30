//! #
use altseed2::prelude::*;
use num::{One, Zero};
use retain_mut::RetainMut;

use std::sync::{Arc, Mutex};

const PLAYER_SCALE: f32 = 0.3;
const PLAYER_SPEED: f32 = 200.0;

const BULLET_SCALE: f32 = 0.1;
const BULLET_SPEED: f32 = 300.0;

const GAME_SIZE: Vector2<f32> = Vector2 { x: 800.0, y: 600.0 };

#[derive(Debug)]
struct Bullet {
    id: DrawnID,
    pos: Vector2<f32>,
    dir: Vector2<f32>,
}

#[derive(Debug)]
struct BulletSystem {
    texture: Arc<Mutex<Texture2D>>,
    texture_size: Vector2<f32>,
    bullets: Vec<Bullet>,
}

impl BulletSystem {
    fn new(engine: &mut Engine) -> AltseedResult<Self> {
        let tex = engine
            .loader()
            .load_texture2d("./Core/TestData/IO/AltseedPink256.png")?;

        let size = tex.lock().unwrap().get_size().into();

        Ok(BulletSystem {
            texture: tex,
            texture_size: size,
            bullets: Vec::new(),
        })
    }

    fn add(&mut self, pos: Vector2<f32>, dir: Vector2<f32>, engine: &mut Engine) {
        let sprite = Sprite::new()
            .with_texture(self.texture.clone())
            .with_scale(Vector2::one() * BULLET_SCALE)
            .with_center(self.texture_size * 0.5)
            .with_pos(pos)
            .build();

        self.bullets.push(Bullet {
            id: engine.drawn_storage_mut().add(sprite),
            pos,
            dir,
        });
    }

    fn update(&mut self, engine: &mut Engine) {
        let size = self.texture_size;
        self.bullets.retain_mut(|bullet| {
            if bullet.pos.x + size.x < 0.0
                || GAME_SIZE.x < bullet.pos.x - size.x
                || bullet.pos.y + size.y < 0.0
                || GAME_SIZE.y < bullet.pos.y - size.y
            {
                return false;
            }

            let delta = engine.get_delta_second();
            bullet.pos += bullet.dir * BULLET_SPEED * delta;

            // 描画の反映
            engine.drawn_storage_mut().with_mut(&bullet.id, |d| {
                let t = d.transform_mut().unwrap();
                *t.pos_mut() = bullet.pos;
                *t.angle_mut() += 10.0 * delta;
            });

            true
        });
    }
}

#[derive(Debug)]
struct Player {
    id: DrawnID,
    pos: Vector2<f32>,
}

impl Player {
    fn new(engine: &mut Engine) -> AltseedResult<Self> {
        let tex = engine
            .loader()
            .load_texture2d("./Core/TestData/IO/AltseedPink256.png")?;

        let tex_size: Vector2<f32> = tex.lock().unwrap().get_size().into();

        let pos = GAME_SIZE * 0.5;

        let sprite = Sprite::new()
            .with_texture(tex)
            .with_center(tex_size * 0.5)
            .with_scale(Vector2::one() * PLAYER_SCALE)
            .with_pos(pos)
            .build();

        let id = engine.drawn_storage_mut().add(sprite);

        Ok(Player { id, pos })
    }

    fn update(&mut self, bullets: &mut BulletSystem, engine: &mut Engine) -> AltseedResult<()> {
        if engine.keyboard().is_push(Keys::Z) {
            bullets.add(self.pos, Vector2::new(0.0, -1.0), engine);
        }

        let mut diff = Vector2::new(0.0, 0.0);

        if engine.keyboard().is_hold(Keys::Right) {
            diff += Vector2::new(1.0, 0.0);
        }

        if engine.keyboard().is_hold(Keys::Left) {
            diff += Vector2::new(-1.0, 0.0);
        }

        if engine.keyboard().is_hold(Keys::Up) {
            diff += Vector2::new(0.0, -1.0);
        }

        if engine.keyboard().is_hold(Keys::Down) {
            diff += Vector2::new(0.0, 1.0);
        }

        if diff != Zero::zero() {
            self.pos += diff * PLAYER_SPEED * engine.get_delta_second();

            // 描画の反映
            engine.drawn_storage_mut().with_mut(&self.id, |d| {
                *d.transform_mut().unwrap().pos_mut() = self.pos;
            });
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Game {
    player: Player,
    bullets: BulletSystem,
}

impl Game {
    fn new(engine: &mut Engine) -> AltseedResult<Self> {
        Ok(Game {
            player: Player::new(engine)?,
            bullets: BulletSystem::new(engine)?,
        })
    }

    fn update(&mut self, engine: &mut Engine) -> AltseedResult<()> {
        let Game { player, bullets } = self;

        player.update(bullets, engine)?;
        bullets.update(engine);

        Ok(())
    }
}

// 返り値に`AltseedResult<()>`を指定して?演算子を使用可能にします。
fn main() -> AltseedResult<()> {
    // Altseedを初期化します。 ?演算子を利用してError時に早期終了します。
    let mut engine = Engine::initialize("sprite", GAME_SIZE.x as i32, GAME_SIZE.y as i32)?;

    let mut game = Game::new(&mut engine)?;

    // 所有権を渡してメインループを実行します。
    // 毎フレーム実行される関数を指定できます。
    engine.run_with(|e| {
        game.update(e)?;
        Ok(())
    })?;

    Ok(())
}
