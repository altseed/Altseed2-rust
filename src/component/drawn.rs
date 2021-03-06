use crate::auto_generated_core_binding::{AsRendered, Graphics, Renderer};

use super::{
    camera::*,
    drawn_kind::*,
    sorted::{Sortable, SortedStorage},
    Component, Entity, Memoried,
};

use crate::math::{HasTransform, Transform};

#[derive(Debug)]
pub struct DrawnComponent {
    kind: DrawnKind,
    is_drawn: bool,
    pub(crate) z_order: i32,
    pub(crate) camera_group: Memoried<u32>,
}

impl Component for DrawnComponent {}

impl Sortable<i32> for DrawnComponent {
    fn key(&self) -> i32 {
        self.z_order
    }
}

use crate::error::*;

impl DrawnComponent {
    pub(crate) fn new(kind: DrawnKind) -> Self {
        DrawnComponent {
            kind,
            is_drawn: true,
            z_order: 0,
            camera_group: Memoried::new(0),
        }
    }

    pub fn kind(&self) -> &DrawnKind {
        &self.kind
    }

    pub fn kind_mut(&mut self) -> &mut DrawnKind {
        &mut self.kind
    }

    pub fn z_order(&self) -> i32 {
        self.z_order
    }

    pub fn z_order_mut(&mut self) -> &mut i32 {
        &mut self.z_order
    }

    pub fn with_z_order(self, z_order: i32) -> Self {
        DrawnComponent { z_order, ..self }
    }

    pub fn camera_group(&self) -> u32 {
        self.camera_group.value()
    }

    pub fn camera_group_mut(&mut self) -> &mut u32 {
        self.camera_group.value_mut()
    }

    pub fn with_camera_group(mut self, camera_group: u32) -> Self {
        *self.camera_group.value_mut() = camera_group;
        self
    }

    pub fn transform(&self) -> Option<&Transform> {
        match &self.kind {
            DrawnKind::Sprite(x) => Some(x.transform()),
            DrawnKind::Text(x) => Some(x.transform()),
            DrawnKind::Polygon(x) => Some(x.transform()),
        }
    }

    pub fn transform_mut(&mut self) -> Option<&mut Transform> {
        match &mut self.kind {
            DrawnKind::Sprite(x) => Some(x.transform_mut()),
            DrawnKind::Text(x) => Some(x.transform_mut()),
            DrawnKind::Polygon(x) => Some(x.transform_mut()),
        }
    }

    pub(crate) fn culling_id(&mut self) -> i32 {
        match &mut self.kind {
            DrawnKind::Sprite(x) => x.instance.get_id(),
            DrawnKind::Text(x) => x.instance.get_id(),
            DrawnKind::Polygon(x) => x.instance.get_id(),
        }
    }

    pub(crate) fn on_drawing(&mut self, entity: Entity, _: &mut CameraStorage) {
        if self.is_drawn {
            match &mut self.kind {
                DrawnKind::Sprite(x) => x.update_transform(),
                DrawnKind::Text(x) => x.update_transform(),
                DrawnKind::Polygon(x) => x.update_transform(),
            }
        }

        if self.camera_group.is_updated() {
            CAMERA_STORAGE.with(|camera_storage| {
                for (_, camera) in camera_storage.borrow_mut().iter_mut() {
                    // カメラのグループが更新されていたらカメラ側でまとめて取り出すので追加しない。
                    if !camera.is_group_updated() {
                        continue;
                    }

                    let group = camera.group();

                    if self.camera_group() & group == group {
                        camera.add_drawn(entity);
                    }
                }
            })
        }
    }

    pub(crate) fn draw(
        &mut self,
        _graphics: &mut Graphics,
        renderer: &mut Renderer,
    ) -> AltseedResult<()> {
        if self.is_drawn {
            match &mut self.kind {
                DrawnKind::Sprite(x) => renderer.draw_sprite(&mut x.instance),
                DrawnKind::Text(x) => renderer.draw_text(&mut x.instance),
                DrawnKind::Polygon(x) => renderer.draw_polygon(&mut x.instance),
            }
        }

        Ok(())
    }

    // cameragroupの更新
    pub(crate) fn update_memoried(&mut self) {
        self.camera_group.update();
    }
}

use std::cell::RefCell;
use std::marker::PhantomData;

thread_local! {
    pub(crate) static DRAWN_STORAGE: RefCell<SortedStorage<DrawnComponent, i32>> = RefCell::new(SortedStorage::new());
}

/// Engineに登録されたDrawnComponentに対応するIDです。
/// このIDがdropされる時、自動的に対応するDrawnComponentも削除されます。
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct DrawnID {
    entity: Entity,
    phantom: PhantomData<*mut ()>,
}

impl Drop for DrawnID {
    fn drop(&mut self) {
        DRAWN_STORAGE.with(|s| {
            s.borrow_mut().remove(self.entity).unwrap();
        });
    }
}

/// DrawnComponentを格納する
#[derive(Debug)]
pub struct DrawnStorage {
    phantom: PhantomData<*mut ()>,
}

impl DrawnStorage {
    pub(crate) fn new() -> Self {
        DRAWN_STORAGE.with(|s| s.borrow_mut().clear());
        DrawnStorage {
            phantom: PhantomData,
        }
    }

    /// IDに対応するDrawnComponentへの参照を取得します。
    #[inline]
    pub fn with<T, F: FnOnce(&DrawnComponent) -> T>(&self, id: &DrawnID, f: F) -> T {
        // DrawnIDが存在を保証しているのでunwrapして良い
        DRAWN_STORAGE.with(|s| f(s.borrow().get(id.entity).unwrap()))
    }

    /// IDに対応するDrawnComponentへの可変参照を取得します。
    #[inline]
    pub fn with_mut<T, F: FnOnce(&mut DrawnComponent) -> T>(&mut self, id: &DrawnID, f: F) -> T {
        // DrawnIDが存在を保証しているのでunwrapして良い
        DRAWN_STORAGE.with(|s| f(s.borrow_mut().get_mut(id.entity).unwrap()))
    }

    /// 即座に新しいDrawnComponentを追加します。
    pub fn add(&mut self, mut component: DrawnComponent) -> DrawnID {
        component.camera_group.reset();
        let entity = DRAWN_STORAGE.with(|s| s.borrow_mut().add(component));
        DrawnID {
            entity,
            phantom: PhantomData,
        }
    }

    /// 即座に要素を削除します。
    pub fn remove(&mut self, id: DrawnID) -> DrawnComponent {
        // DrawnIDが存在を保証しているのでunwrapして良い
        let mut res = DRAWN_STORAGE
            .with(|s| s.borrow_mut().remove(id.entity))
            .unwrap();
        res.camera_group.reset();
        // removeしてあるのでdrop処理を行う必要はない
        std::mem::forget(id);
        res
    }

    /// 即座に全ての要素を削除します。
    pub fn clear(&mut self) {
        DRAWN_STORAGE.with(|s| s.borrow_mut().clear());
    }

    /// 現在の要素数を取得します。
    pub fn len(&self) -> usize {
        DRAWN_STORAGE.with(|s| s.borrow().len())
    }

    #[inline]
    pub fn create_sprite<F: FnOnce(Sprite) -> DrawnComponent>(&mut self, f: F) -> DrawnID {
        self.add(f(Sprite::new()))
    }

    #[inline]
    pub fn create_text<F: FnOnce(Text) -> DrawnComponent>(&mut self, f: F) -> DrawnID {
        self.add(f(Text::new()))
    }

    #[inline]
    pub fn create_polygon<F: FnOnce(Polygon) -> DrawnComponent>(&mut self, f: F) -> DrawnID {
        self.add(f(Polygon::new()))
    }
}
