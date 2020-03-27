use crate::collections::RetainMutResult;
use crate::retain_mut::RetainMut;

use super::{Component, Entity, Memoried};

pub trait Sortable<T: PartialEq + Ord + Clone + Copy> {
    fn key(&self) -> &Memoried<T>;
    fn key_mut(&mut self) -> &mut Memoried<T>;
}

impl<T: Sortable<U>, U: PartialEq + Ord + Clone + Copy> Sortable<U> for (Entity, T) {
    fn key(&self) -> &Memoried<U> {
        self.1.key()
    }

    fn key_mut(&mut self) -> &mut Memoried<U> {
        self.1.key_mut()
    }
}

#[derive(Debug, Clone)]
pub struct LazySortVec<T, U: PartialEq + Ord + Clone + Copy> {
    items: Vec<T>,
    last_key: Option<U>,
    sort_needed: bool,
}

impl<T, U> RetainMut<T> for LazySortVec<T, U>
where
    U: PartialEq + Ord + Clone + Copy,
{
    fn retain_mut<F: FnMut(&mut T) -> bool>(&mut self, f: F) {
        self.items.retain_mut(f);
    }
}

impl<T, U> RetainMutResult<T> for LazySortVec<T, U>
where
    U: PartialEq + Ord + Clone + Copy,
{
    fn retain_mut_result<E, F: FnMut(&mut T) -> Result<bool, E>>(&mut self, f: F) -> Result<(), E> {
        self.items.retain_mut_result(f)
    }
}

impl<T, U> LazySortVec<T, U>
where
    U: PartialEq + Ord + Clone + Copy,
{
    pub fn new() -> Self {
        LazySortVec {
            items: Vec::new(),
            last_key: None,
            sort_needed: false,
        }
    }

    pub fn sort_needed(&self) -> bool {
        self.sort_needed
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.items.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.items.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.items.get_mut(index)
    }

    pub fn push_with(&mut self, item: T, key: U) {
        if !self.sort_needed {
            match self.last_key {
                Some(x) if x > key => {
                    self.sort_needed = true;
                }
                _ => (),
            }
        }

        self.last_key = Some(key);
        self.items.push(item);
    }

    pub fn sort_by_key<F: FnMut(&T) -> U>(&mut self, f: F) {
        self.items.sort_by_key(f);
        self.sort_needed = false;
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.last_key = None;
        self.sort_needed = false;
    }
}

impl<T, U> LazySortVec<T, U>
where
    T: Sortable<U>,
    U: PartialEq + Ord + Clone + Copy,
{
    pub fn push(&mut self, item: T) {
        let key = item.key().value();

        if !self.sort_needed {
            match self.last_key {
                Some(x) if x > key => {
                    self.sort_needed = true;
                }
                _ => (),
            }
        }

        self.last_key = Some(key);
        self.items.push(item);
    }

    // returns whether the sort has been performed or not
    pub fn update<F: FnMut(&mut T) -> bool>(&mut self, mut f: F) -> bool {
        let mut key_updated = false;
        self.items.retain_mut(|x| {
            let v = f(x);
            key_updated = x.key_mut().is_updated();
            v
        });

        if self.sort_needed || key_updated {
            self.items.sort_by_key(|x| x.key().value());
            self.sort_needed = false;
            true
        } else {
            false
        }
    }
}

use std::collections::HashMap;

#[derive(Debug)]
pub struct SortedStorage<T, U>
where
    T: Sortable<U> + Component,
    U: PartialEq + Ord + Clone + Copy,
{
    next_id: u32,
    removed_entities: Vec<Entity>,
    indexes: HashMap<Entity, u32>,
    components: LazySortVec<(Entity, T), U>,
}

impl<T, U> SortedStorage<T, U>
where
    T: Sortable<U> + Component,
    U: PartialEq + Ord + Clone + Copy,
{
    pub fn new() -> Self {
        SortedStorage {
            next_id: 0,
            removed_entities: Vec::new(),
            indexes: HashMap::new(),
            components: LazySortVec::new(),
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, (Entity, T)> {
        self.components.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, (Entity, T)> {
        self.components.iter_mut()
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.indexes.get(&entity).is_some()
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        let index = self.indexes.get(&entity)?;

        self.components.get(*index as usize).map(|x| &x.1)
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        let index = self.indexes.get(&entity)?;
        self.components.get_mut(*index as usize).map(|x| &mut x.1)
    }

    /// 新しい要素を追加します。
    /// 追加された要素にアクセスするためのEntityを返します。
    pub fn add(&mut self, item: T) -> Entity {
        let entity = match self.removed_entities.pop() {
            Some(e) => Entity {
                version: e.version + 1,
                ..e
            },
            None => {
                let id = self.next_id;
                self.next_id += 1;
                Entity { id, version: 0 }
            }
        };

        let index = self.components.len();
        self.indexes.insert(entity, index as u32);

        self.components.push((entity, item));

        entity
    }

    /// 削除のフラグを立てます。実際の反映は更新時まで遅延されます。
    pub fn remove(&mut self, entity: Entity) -> bool {
        match self.get_mut(entity) {
            Some(d) => {
                *d.alive_mut() = false;
                true
            }
            None => false,
        }
    }

    fn update_components(&mut self) -> bool {
        let SortedStorage {
            components,
            indexes,
            removed_entities,
            next_id: _,
        } = self;

        let mut key_updated = false;
        let sorted = components.update(|(entity, x)| {
            let v = x.alive();
            if !v {
                indexes.remove(entity);
                removed_entities.push(*entity);
            }
            key_updated = x.key_mut().is_updated();
            v
        });

        sorted || key_updated
    }

    /// 更新する
    pub fn update(&mut self) {
        let sorted_needed = self.update_components();

        if sorted_needed {
            let mut index = 0;
            for (entity, _) in self.components.iter_mut() {
                *self.indexes.get_mut(entity).unwrap() = index;
                index += 1;
            }
        }
    }

    /// 全てのComponentに実行する関数を指定して更新する
    pub fn update_with<E, F>(&mut self, mut f: F) -> Result<(), E>
    where
        F: FnMut(&Entity, &mut T) -> Result<(), E>,
    {
        let sorted_needed = self.update_components();

        if sorted_needed {
            let mut index = 0;
            for (entity, component) in self.components.iter_mut() {
                f(entity, component)?;
                *self.indexes.get_mut(entity).unwrap() = index;
                index += 1;
            }
        } else {
            for (entity, component) in self.components.iter_mut() {
                f(entity, component)?;
            }
        }

        Ok(())
    }
}
