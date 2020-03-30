//! #
use altseed2::prelude::*;
use num::One;
use retain_mut::RetainMut;

use std::sync::{Arc, Mutex};
use std::{cell::RefCell, rc::Rc};

const PLAYER_SCALE: f32 = 0.3;
const PLAYER_SPEED: f32 = 200.0;

const BULLET_SCALE: f32 = 0.1;
const BULLET_SPEED: f32 = 300.0;

const GAME_SIZE: Vector2<f32> = Vector2 { x: 800.0, y: 600.0 };

#[derive(Debug)]
struct GameResources {
    player: Rc<RefCell<Texture2D>>,
    bullet: Rc<RefCell<Texture2D>>,
}

#[derive(Debug)]
struct Player {
    id: DrawnID,
    pos: Vector2<f32>,
}

impl Player {
    fn new(res: &GameResources, engine: &mut Engine) -> Self {
        let tex_size: Vector2<f32> = res.player.borrow_mut().get_size().into();
        let sprite = Sprite::new()
            .with_texture(res.player.clone())
            .with_center(tex_size * 0.5)
            .with_scale(Vector2::one() * PLAYER_SCALE)
            .build();
        Player {
            id: engine.drawn_storage_mut().add(sprite),
            pos: GAME_SIZE * 0.5,
        }
    }

    fn update(
        &mut self,
        bullets: &mut Vec<Bullet>,
        res: &GameResources,
        engine: &mut Engine,
    ) -> AltseedResult<()> {
        let mut diff = Vector2::new(0.0, 0.0);
        if engine.keyboard().get_key_state(Keys::Right) == ButtonState::Hold {
            diff += Vector2::new(1.0, 0.0);
        }

        if engine.keyboard().get_key_state(Keys::Left) == ButtonState::Hold {
            diff += Vector2::new(-1.0, 0.0);
        }

        if engine.keyboard().get_key_state(Keys::Up) == ButtonState::Hold {
            diff += Vector2::new(0.0, -1.0);
        }

        if engine.keyboard().get_key_state(Keys::Down) == ButtonState::Hold {
            diff += Vector2::new(0.0, 1.0);
        }

        self.pos += diff * PLAYER_SPEED * engine.get_delta_second();

        if engine.keyboard().get_key_state(Keys::Z) == ButtonState::Push {
            bullets.push(Bullet::new(self.pos, Vector2::new(0.0, -1.0), res, engine)?);
        }

        engine.drawn_storage_mut().with_mut(&self.id, |d| {
            *d.transform_mut().unwrap().pos_mut() = self.pos;
        });

        Ok(())
    }
}

#[derive(Debug)]
struct Bullet {
    id: DrawnID,
    pos: Vector2<f32>,
    dir: Vector2<f32>,
    size: Vector2<f32>,
}

impl Bullet {
    fn new(
        pos: Vector2<f32>,
        dir: Vector2<f32>,
        res: &GameResources,
        engine: &mut Engine,
    ) -> AltseedResult<Self> {
        let tex_size: Vector2<f32> = res.bullet.borrow_mut().get_size().into();
        let sprite = Sprite::new()
            .with_texture(res.bullet.clone())
            .with_scale(Vector2::one() * BULLET_SCALE)
            .with_center(tex_size * 0.5)
            .build();
        Ok(Bullet {
            id: engine.drawn_storage_mut().add(sprite),
            pos,
            dir,
            size: tex_size * BULLET_SCALE,
        })
    }

    fn update(&mut self, engine: &mut Engine) -> bool {
        self.pos += self.dir * BULLET_SPEED * engine.get_delta_second();

        if self.pos.x + self.size.x < 0.0
            || GAME_SIZE.x < self.pos.x - self.size.x
            || self.pos.y + self.size.y < 0.0
            || GAME_SIZE.y < self.pos.y - self.size.y
        {
            return false;
        }

        engine.drawn_storage_mut().with_mut(&self.id, |d| {
            *d.transform_mut().unwrap().pos_mut() = self.pos;
        });

        true
    }
}

#[derive(Debug)]
struct Game {
    res: GameResources,
    player: Player,
    bullets: Vec<Bullet>,
}

impl Game {
    fn new(engine: &mut Engine) -> AltseedResult<Self> {
        // 画像を読み込みます。
        let tex = engine
            .loader()
            .load_texture2d("./Core/TestData/IO/AltseedPink256.png")?;

        let res = GameResources {
            player: tex.clone(),
            bullet: tex,
        };

        let player = Player::new(&res, engine);

        Ok(Game {
            res: res,
            player: player,
            bullets: Vec::new(),
        })
    }

    fn update(&mut self, engine: &mut Engine) -> AltseedResult<()> {
        let Game {
            res,
            player,
            bullets,
        } = self;

        player.update(bullets, res, engine)?;
        bullets.retain_mut(|b| b.update(engine));

        if engine.keyboard().get_key_state(Keys::Space) == ButtonState::Push {
            println!("{:?}", bullets);
        }

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
