use crate::prelude::*;

#[test]
fn transform() {
    let t = Transform::default();

    assert_eq!(t.get(), Matrix44::identity());
}

#[test]
fn transform_pos() {
    let mut t = Transform::default();
    let pos = Vector2::new(1.0, 2.0);
    *t.pos_mut() = pos;
    t.update();

    assert_eq!(t.get(), Matrix44::translation(pos.x, pos.y, 0.0));
}
