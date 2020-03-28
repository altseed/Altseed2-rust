use crate::collections::RetainMutResult;
use crate::retain_mut::RetainMut;

use super::{Component, Entity};

pub trait Sortable<T: PartialEq + Ord + Clone + Copy> {
    fn key(&self) -> T;
    fn is_key_updated(&self) -> bool;
}

impl<T: Sortable<U>, U: PartialEq + Ord + Clone + Copy + Default> Sortable<U>
    for Option<(Entity, T)>
{
    fn key(&self) -> U {
        self.as_ref().map(|x| x.1.key()).unwrap_or_default()
    }

    fn is_key_updated(&self) -> bool {
        self.as_ref().map(|x| x.1.is_key_updated()).unwrap_or(false)
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

    pub fn sort_by_key<F: FnMut(&T) -> U + Clone>(&mut self, mut f: F) {
        self.items.sort_by_key(f.clone());
        self.last_key = Some(f(self.items.last().unwrap()));
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
        let key = item.key();

        if !self.sort_needed {
            match self.last_key {
                Some(x) if x > key => {
                    self.sort_needed = true;
                }
                _ => {
                    self.last_key = Some(key);
                }
            }
        }

        self.items.push(item);
    }

    /// ソートが行われたかどうかを返す
    pub fn update<F: FnMut(&mut T) -> bool>(&mut self, mut f: F) -> bool {
        let mut key_updated = false;
        self.items.retain_mut(|x| {
            let v = f(x);
            key_updated = x.is_key_updated();
            v
        });

        if self.sort_needed || key_updated {
            self.items.sort_by_key(|x| x.key());
            self.last_key = Some(self.items.last().unwrap().key());
            self.sort_needed = false;
            true
        } else {
            false
        }
    }
}

pub struct Iter<'a, T: Component> {
    components: std::slice::Iter<'a, Option<(Entity, T)>>,
}

impl<'a, T: Component> std::iter::Iterator for Iter<'a, T> {
    type Item = (Entity, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(container) = self.components.next() {
            if let Some(x) = container {
                return Some((x.0, &x.1));
            }
        }

        None
    }
}

pub struct IterMut<'a, T: Component> {
    components: std::slice::IterMut<'a, Option<(Entity, T)>>,
}

impl<'a, T: Component> std::iter::Iterator for IterMut<'a, T> {
    type Item = (Entity, &'a mut T);
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(container) = self.components.next() {
            if let Some(x) = container {
                return Some((x.0, &mut x.1));
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct SortedStorage<T, U>
where
    T: Sortable<U> + Component,
    U: PartialEq + Ord + Clone + Copy + Default,
{
    next_index: u32,
    removed_entities: Vec<Entity>,
    indexes: Vec<Entity>,
    components: LazySortVec<Option<(Entity, T)>, U>,
}

impl<T, U> SortedStorage<T, U>
where
    T: Sortable<U> + Component,
    U: PartialEq + Ord + Clone + Copy + Default,
{
    pub fn new() -> Self {
        SortedStorage {
            next_index: 0,
            removed_entities: Vec::new(),
            indexes: Vec::new(),
            components: LazySortVec::new(),
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            components: self.components.iter(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            components: self.components.iter_mut(),
        }
    }

    pub fn contains(&self, entity: Entity) -> bool {
        match self.indexes.get(entity.index as usize) {
            Some(e) if e.version == entity.version => true,
            _ => false,
        }
    }

    pub fn len(&self) -> u32 {
        self.next_index - self.removed_entities.len() as u32
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        let e = self.indexes.get(entity.index as usize)?;
        if e.version != entity.version {
            return None;
        }

        self.components
            .get(e.index as usize)?
            .as_ref()
            .map(|x| &x.1)
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        let e = self.indexes.get(entity.index as usize)?;
        if e.version != entity.version {
            return None;
        }

        self.components
            .get_mut(e.index as usize)?
            .as_mut()
            .map(|x| &mut x.1)
    }

    /// 新しい要素を追加します。
    /// 追加された要素にアクセスするためのEntityを返します。
    pub fn add(&mut self, item: T) -> Entity {
        let components_index = self.components.len() as u32;

        let entity = match self.removed_entities.pop() {
            Some(entity) => {
                self.indexes[entity.index as usize] = Entity {
                    index: components_index,
                    ..entity
                };
                entity
            }
            None => {
                let index = self.next_index;
                self.next_index += 1;
                let entity = Entity { index, version: 0 };
                self.indexes.push(Entity {
                    index: components_index,
                    ..entity
                });
                entity
            }
        };
        let key = item.key();
        self.components.push_with(Some((entity, item)), key);
        entity
    }

    /// 要素を削除します。削除に成功したら格納されていた要素を返します。
    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        match self.indexes.get_mut(entity.index as usize) {
            Some(e) if e.version == entity.version => {
                e.version += 1;
                let new_entity = Entity {
                    version: entity.version + 1,
                    ..entity
                };
                self.removed_entities.push(new_entity);
                self.components
                    .get_mut(e.index as usize)?
                    .take()
                    .map(|x| x.1)
            }
            _ => None,
        }
    }

    /// 全ての要素を削除します。
    pub fn clear(&mut self) {
        for x in self.components.iter() {
            if let Some((e, _)) = x {
                self.removed_entities.push(*e);
            }
        }
        self.components.clear();
    }

    /// 更新する
    pub fn update(&mut self) {
        // ソートが行われたかどうか
        let is_sort_performed = self.components.update(|c| c.is_some());

        if is_sort_performed {
            let mut index = 0;
            for c in self.components.iter_mut() {
                // update_componentsでretain済みなのでunwrap
                let (entity, _) = c.as_mut().unwrap();
                self.indexes.get_mut(entity.index as usize).unwrap().index = index;
                index += 1;
            }
        }
    }

    /// 全てのComponentに実行する関数を指定して更新する
    pub fn update_with<E, F>(&mut self, mut f: F) -> Result<(), E>
    where
        F: FnMut(&Entity, &mut T) -> Result<(), E>,
    {
        // ソートが行われたかどうか
        let is_sort_performed = self.components.update(|c| c.is_some());

        if is_sort_performed {
            let mut index = 0;
            for c in self.components.iter_mut() {
                // update_componentsでretain済みなのでunwrap
                let (entity, comp) = c.as_mut().unwrap();
                f(entity, comp)?;
                self.indexes.get_mut(entity.index as usize).unwrap().index = index;
                index += 1;
            }
        } else {
            for c in self.components.iter_mut() {
                // update_componentsでretain済みなのでunwrap
                let (entity, comp) = c.as_mut().unwrap();
                f(entity, comp)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Component, Memoried};
    use super::*;

    #[derive(Debug)]
    struct TestC {
        key: Memoried<i32>,
    }

    impl Sortable<i32> for TestC {
        fn key(&self) -> i32 {
            self.key.value()
        }

        fn is_key_updated(&self) -> bool {
            self.key.is_updated()
        }
    }

    impl Component for TestC {}

    impl TestC {
        fn new() -> Self {
            TestC {
                key: Memoried::new(0),
            }
        }
    }

    #[test]
    fn add_remove_sorted_storage() {
        let mut storage = SortedStorage::new();

        let _e1 = storage.add(TestC::new());
        let e2 = storage.add(TestC::new());
        let _e3 = Some(storage.add(TestC::new()));

        assert_eq!(storage.len(), 3);
        assert_eq!(storage.indexes.len(), 3);
        assert_eq!(storage.components.len(), 3);

        storage.remove(e2);
        assert_eq!(storage.len(), 2);
        assert_eq!(storage.indexes.len(), 3);
        assert_eq!(storage.components.len(), 3);

        storage.update();

        assert_eq!(storage.indexes.len(), 3);
        assert_eq!(storage.components.len(), 2);
    }
}
