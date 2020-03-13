use crate::prelude::*;

use std::f32::consts::PI;

#[test]
fn vector_dot() {
    let a = Vector3::new(1.0, 2.0, 3.0);
    let b = Vector3::new(4.0, 5.0, 6.0);
    let c = Vector3::new(7.0, 8.0, 9.0);

    assert_eq!((a * b) * c, a * (b * c));
}

#[test]
fn matrix_dot() {
    let a = Matrix44::translation(Vector3::new(1.0, 2.0, 3.0));
    let b = Matrix44::rotation_z(5.0);
    let c = Matrix44::scale(Vector3::new(7.0, 8.0, 9.0));

    assert_eq!((&a * &b) * c.clone(), a * (b * c));
}

#[test]
fn translation() {
    let a = Matrix44::translation(Vector3::new(1.0, 2.0, 3.0));
    let x = Vector3::new(-1.0, 5.0, 9.0);
    let transformed = a.transform_3d(x);
    let expected = Vector3::new(0.0, 7.0, 12.0);
    assert!((transformed.x - expected.x).abs() < 0.0001);
    assert!((transformed.y - expected.y).abs() < 0.0001);
    assert!((transformed.z - expected.z).abs() < 0.0001);
}

#[test]
fn rotation() {
    let a = Matrix44::rotation_z(PI * 0.5);
    let x = Vector3::new(1.0, 0.0, 0.0);
    let transformed = a.transform_3d(x);
    let expected = Vector3::new(0.0, 1.0, 0.0);
    assert!((transformed.x - expected.x).abs() < 0.0001);
    assert!((transformed.y - expected.y).abs() < 0.0001);
    assert!((transformed.z - expected.z).abs() < 0.0001);
}
