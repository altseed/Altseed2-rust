use crate::collections::RetainMutResult;
use crate::retain_mut::RetainMut;

use super::{Component, ComponentContainer, Entity, Memoried};

pub trait Sortable<T: PartialEq + Ord + Clone + Copy> {
    fn key(&self) -> &Memoried<T>;
    fn key_mut(&mut self) -> &mut Memoried<T>;
}

impl<T: Component + Sortable<U>, U: PartialEq + Ord + Clone + Copy> Sortable<U>
    for ComponentContainer<T>
{
    fn key(&self) -> &Memoried<U> {
        self.data.key()
    }

    fn key_mut(&mut self) -> &mut Memoried<U> {
        self.data.key_mut()
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

pub struct Iter<'a, T: Component> {
    components: std::slice::Iter<'a, ComponentContainer<T>>,
}

impl<'a, T: Component> std::iter::Iterator for Iter<'a, T> {
    type Item = (Entity, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(container) = self.components.next() {
            if container.alive {
                return Some((container.entity, &container.data));
            }
        }

        None
    }
}

pub struct IterMut<'a, T: Component> {
    components: std::slice::IterMut<'a, ComponentContainer<T>>,
}

impl<'a, T: Component> std::iter::Iterator for IterMut<'a, T> {
    type Item = (Entity, &'a mut T);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(container) = self.components.next() {
            if container.alive {
                return Some((container.entity, &mut container.data));
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct SortedStorage<T, U>
where
    T: Sortable<U> + Component,
    U: PartialEq + Ord + Clone + Copy,
{
    next_id: u32,
    removed_entities: Vec<Entity>,
    indexes: HashMap<Entity, u32>,
    components: LazySortVec<ComponentContainer<T>, U>,
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

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            components: self.components.iter(),
        }
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, ComponentContainer<T>> {
        self.components.iter_mut()
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.indexes.get(&entity).is_some()
    }

    pub fn len(&self) -> u32 {
        self.indexes.len() as u32
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        let index = self.indexes.get(&entity)?;

        self.components.get(*index as usize).map(|x| &x.data)
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        let index = self.indexes.get(&entity)?;
        self.components
            .get_mut(*index as usize)
            .map(|x| &mut x.data)
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

        self.components.push(ComponentContainer::new(entity, item));

        entity
    }

    /// 要素を削除します。削除に成功したらtrueを返します。
    pub fn remove(&mut self, entity: Entity) -> bool {
        if let Some(index) = self.indexes.remove(&entity) {
            self.removed_entities.push(entity);
            let container = self.components.get_mut(index as usize).unwrap();
            container.alive = false;
            true
        } else {
            false
        }
    }

    /// 全ての要素を削除します。
    pub fn clear(&mut self) {
        self.components.clear();
        for (entity, _) in self.indexes.iter() {
            self.removed_entities.push(*entity);
        }
        self.indexes.clear();
    }

    fn update_components(&mut self) -> bool {
        let SortedStorage {
            components,
            indexes,
            removed_entities,
            next_id: _,
        } = self;

        let mut key_updated = false;
        let sorted = components.update(|c| {
            let v = c.alive;
            if !v {
                indexes.remove(&c.entity);
                removed_entities.push(c.entity);
            }
            key_updated = c.data.key_mut().is_updated();
            v
        });

        sorted || key_updated
    }

    /// 更新する
    pub fn update(&mut self) {
        let sorted_needed = self.update_components();

        if sorted_needed {
            let mut index = 0;
            for c in self.components.iter_mut() {
                *self.indexes.get_mut(&c.entity).unwrap() = index;
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
            for c in self.components.iter_mut() {
                f(&c.entity, &mut c.data)?;
                *self.indexes.get_mut(&c.entity).unwrap() = index;
                index += 1;
            }
        } else {
            for c in self.components.iter_mut() {
                f(&c.entity, &mut c.data)?;
            }
        }

        Ok(())
    }
}
