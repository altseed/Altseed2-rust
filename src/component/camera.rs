use crate::retain_mut::RetainMut;

use super::{
    drawn::*,
    sorted::{LazySortVec, Sortable, SortedStorage},
    Component, Entity, Memoried,
};

use crate::auto_generated_core_binding::{Graphics, RenderedCamera, Renderer};

use crate::error::*;

#[derive(Debug)]
pub struct CameraComponent {
    instance: RenderedCamera,
    group: Memoried<u32>,
    drawn_entities: LazySortVec<Entity, i32>,
}

impl Component for CameraComponent {}

impl Sortable<u32> for CameraComponent {
    fn key(&self) -> u32 {
        self.group.value()
    }

    fn is_key_updated(&self) -> bool {
        self.group.is_updated()
    }
}

impl CameraComponent {
    pub fn new() -> Self {
        CameraComponent {
            instance: RenderedCamera::create().unwrap(),
            group: Memoried::new(0),
            drawn_entities: LazySortVec::new(),
        }
    }

    pub fn group(&self) -> u32 {
        self.group.value()
    }

    pub fn group_mut(&mut self) -> &mut u32 {
        self.group.value_mut()
    }

    pub fn with_group(mut self, group: u32) -> Self {
        *self.group.value_mut() = group;
        self
    }

    pub(crate) fn add_drawn(&mut self, id: Entity, z_order: i32) {
        self.drawn_entities.push_with(id, z_order);
    }

    /// 呼び出し前にDrawnComponentは更新済みであることを想定している。
    pub(crate) fn draw(
        &mut self,
        _: &mut DrawnStorage,
        graphics: &mut Graphics,
        renderer: &mut Renderer,
    ) -> AltseedResult<()> {
        let group = self.group();

        DRAWN_STORAGE.with(|drawn_storage| {
            let mut storage = drawn_storage.borrow_mut();
            // 自身のカメラグループが更新されているかどうか
            if self.group.is_updated() {
                self.drawn_entities.clear();

                // DrawnComponentはソート済みのはずなので
                for (e, d) in storage
                    .iter()
                    .filter(|(_, d)| d.camera_group() & group == group)
                {
                    self.drawn_entities.push_with(e, d.z_order());
                }
            } else {
                let mut sort_needed = false;
                self.drawn_entities.retain_mut(|e| {
                    // 生存しており、カメラグループが合致しているものだけを取り出す。
                    if let Some(d) = storage.get_mut(*e) {
                        sort_needed = d.is_key_updated();
                        d.camera_group() & group == group
                    } else {
                        false
                    }
                });

                if sort_needed || self.drawn_entities.sort_needed() {
                    self.drawn_entities
                        .sort_by_key(|e| storage.get(*e).unwrap().key());
                }

                self.group.update();

                renderer.set_camera(&mut self.instance);

                for e in self.drawn_entities.iter() {
                    let d = storage.get_mut(*e).unwrap();
                    d.draw(graphics, renderer)?;
                }
            }
            Ok(())
        })
    }
}

use std::cell::RefCell;
use std::marker::PhantomData;

thread_local! {
    pub(crate) static CAMERA_STORAGE: RefCell<SortedStorage<CameraComponent, u32>> = RefCell::new(SortedStorage::new());
}

/// Engineに登録されたCameraComponentに対応するIDです。
/// このIDがdropされる時、自動的に対応するCameraComponentも削除されます。
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct CameraID {
    entity: Entity,
    phantom: PhantomData<*mut ()>,
}

impl Drop for CameraID {
    fn drop(&mut self) {
        CAMERA_STORAGE.with(|s| s.borrow_mut().remove(self.entity));
    }
}

/// CameraComponentを格納します。
#[derive(Debug)]
pub struct CameraStorage {
    // pub(crate) storage: SortedStorage<CameraComponent, u32>,
    phantom: PhantomData<()>,
}

impl CameraStorage {
    pub(crate) fn new() -> Self {
        CameraStorage {
            // storage: SortedStorage::new(),
            phantom: PhantomData,
        }
    }

    /// IDに対応するCameraComponentへの参照を取得します。
    pub fn with<T, F: FnOnce(&CameraComponent) -> T>(&self, id: &CameraID, f: F) -> T {
        // CameraIDが存在を保証しているのでunwrapして良い
        CAMERA_STORAGE.with(|s| f(s.borrow().get(id.entity).unwrap()))
    }

    /// IDに対応するCameraComponentへの可変参照を取得します。
    pub fn with_mut<T, F: FnOnce(&mut CameraComponent) -> T>(&mut self, id: &CameraID, f: F) -> T {
        // CameraIDが存在を保証しているのでunwrapして良い
        CAMERA_STORAGE.with(|s| f(s.borrow_mut().get_mut(id.entity).unwrap()))
    }

    /// 即座に新しいCameraComponentを追加します。
    pub fn add(&mut self, component: CameraComponent) -> CameraID {
        let entity = CAMERA_STORAGE.with(|s| s.borrow_mut().add(component));
        CameraID {
            entity,
            phantom: PhantomData,
        }
    }

    /// 即座に要素を削除します。
    pub fn remove(&mut self, id: CameraID) -> CameraComponent {
        // CameraIDが存在を保証しているのでunwrapして良い
        let res = CAMERA_STORAGE
            .with(|s| s.borrow_mut().remove(id.entity))
            .unwrap();
        // removeしてあるのでdrop処理を行う必要はない
        std::mem::forget(id);
        res
    }

    /// 即座に全ての要素を削除します。
    pub fn clear(&mut self) {
        CAMERA_STORAGE.with(|s| s.borrow_mut().clear());
    }

    /// 現在の要素数を取得します。
    pub fn len(&self) -> u32 {
        CAMERA_STORAGE.with(|s| s.borrow().len())
    }
}
