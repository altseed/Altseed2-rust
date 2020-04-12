use super::{storage::Storage, Component, Entity, Memoried};
use crate::math::Vector2;

#[derive(Debug)]
pub struct ColliderComponent {
    pos: Memoried<Vector2<f32>>,
    angle: Memoried<f32>,
}

impl Component for ColliderComponent {}

use std::cell::RefCell;
use std::marker::PhantomData;

thread_local! {
    pub(crate) static COLLIDER_STORAGE: RefCell<Storage<ColliderComponent>> = RefCell::new(Storage::new());
}

/// Engineに登録されたColliderComponentに対応するIDです。
/// このIDがdropされる時、自動的に対応するColliderComponentも削除されます。
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ColliderID {
    entity: Entity,
    phantom: PhantomData<*mut ()>,
}

impl Drop for ColliderID {
    fn drop(&mut self) {
        COLLIDER_STORAGE.with(|s| {
            s.borrow_mut().remove(self.entity).unwrap();
        });
    }
}

#[derive(Debug)]
pub struct ColliderStorage {
    phantom: PhantomData<*mut ()>,
}

impl ColliderStorage {
    pub(crate) fn new() -> Self {
        COLLIDER_STORAGE.with(|s| s.borrow_mut().clear());
        ColliderStorage {
            phantom: PhantomData,
        }
    }

    /// IDに対応するColliderComponentへの参照を取得します。
    pub fn with<T, F: FnOnce(&ColliderComponent) -> T>(&self, id: &ColliderID, f: F) -> T {
        // ColliderIDが存在を保証しているのでunwrapして良い
        COLLIDER_STORAGE.with(|s| f(s.borrow().get(id.entity).unwrap()))
    }

    /// IDに対応するColliderComponentへの可変参照を取得します。
    pub fn with_mut<T, F: FnOnce(&mut ColliderComponent) -> T>(
        &mut self,
        id: &ColliderID,
        f: F,
    ) -> T {
        // ColliderIDが存在を保証しているのでunwrapして良い
        COLLIDER_STORAGE.with(|s| f(s.borrow_mut().get_mut(id.entity).unwrap()))
    }

    /// 即座に新しいColliderComponentを追加します。
    pub fn add(&mut self, component: ColliderComponent) -> ColliderID {
        let entity = COLLIDER_STORAGE.with(|s| s.borrow_mut().add(component));
        ColliderID {
            entity,
            phantom: PhantomData,
        }
    }

    /// 即座に要素を削除します。
    pub fn remove(&mut self, id: ColliderID) -> ColliderComponent {
        // ColliderIDが存在を保証しているのでunwrapして良い
        let res = COLLIDER_STORAGE
            .with(|s| s.borrow_mut().remove(id.entity))
            .unwrap();
        // removeしてあるのでdrop処理を行う必要はない
        std::mem::forget(id);
        res
    }

    /// 即座に全ての要素を削除します。
    pub fn clear(&mut self) {
        COLLIDER_STORAGE.with(|s| s.borrow_mut().clear());
    }

    /// 現在の要素数を取得します。
    pub fn len(&self) -> usize {
        COLLIDER_STORAGE.with(|s| s.borrow().len())
    }
}
