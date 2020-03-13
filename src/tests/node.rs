use crate::prelude::*;

#[test]
fn transform() {
    let t = Transform::default();

    assert_eq!(t.get(), Matrix44::identity());
}
