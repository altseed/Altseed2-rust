use crate::retain_mut::RetainMut;

use super::{Component, Entity};

pub trait Sortable<T: Ord + Clone + Copy> {
    fn key(&self) -> T;
}

impl<T: Sortable<U>, U: Ord + Clone + Copy + Default> Sortable<U> for Option<(Entity, T)> {
    fn key(&self) -> U {
        self.as_ref().map(|x| x.1.key()).unwrap_or_default()
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

use std::marker::PhantomData;

/// 更新時まで遅延されてまとめてソートされるComponent Storage
#[derive(Debug)]
pub struct SortedStorage<T, U>
where
    T: Sortable<U> + Component,
    U: Ord + Clone + Copy + Default,
{
    removed_entities: Vec<Entity>,
    indexes: Vec<Entity>,
    components: Vec<Option<(Entity, T)>>,
    phantom: PhantomData<U>,
}

impl<T, U> SortedStorage<T, U>
where
    T: Sortable<U> + Component,
    U: Ord + Clone + Copy + Default,
{
    pub fn new() -> Self {
        SortedStorage {
            removed_entities: Vec::new(),
            indexes: Vec::new(),
            components: Vec::new(),
            phantom: PhantomData,
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

    pub fn len(&self) -> usize {
        self.indexes.len() - self.removed_entities.len()
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        let e = self.indexes.get(entity.index as usize)?;
        if e.version != entity.version {
            return None;
        }

        self.components[e.index as usize].as_ref().map(|x| &x.1)
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        let e = self.indexes.get(entity.index as usize)?;
        if e.version != entity.version {
            return None;
        }

        self.components[e.index as usize].as_mut().map(|x| &mut x.1)
    }

    /// 新しい要素を追加します。
    /// 追加された要素にアクセスするためのEntityを返します。
    pub fn add(&mut self, item: T) -> Entity {
        let components_index = self.components.len() as u32;

        let entity = match self.removed_entities.pop() {
            Some(entity) => {
                self.indexes[entity.index as usize].index = components_index;
                entity
            }
            None => {
                let index = self.indexes.len() as u32;

                self.indexes.push(Entity {
                    index: components_index,
                    version: 0,
                });

                Entity { index, version: 0 }
            }
        };

        self.components.push(Some((entity, item)));
        entity
    }

    /// 要素を削除します。削除に成功したら格納されていた要素を返します。
    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        let e = self.indexes.get_mut(entity.index as usize)?;

        if e.version != entity.version {
            return None;
        }

        let (_, res) = self.components[e.index as usize].take()?;

        e.version += 1;

        self.removed_entities.push(Entity {
            version: entity.version + 1,
            ..entity
        });

        Some(res)
    }

    /// 全ての要素を削除します。
    pub fn clear(&mut self) {
        while let Some(x) = self.components.pop() {
            if let Some((e, _)) = x {
                self.removed_entities.push(e);
            }
        }
    }

    /// 更新してソートが行われたかどうかを返す
    fn update_components(&mut self) -> bool {
        let mut last_key = None;
        let mut sort_needed = false;
        let SortedStorage {
            indexes,
            components,
            removed_entities: _,
            phantom: _,
        } = self;

        let mut index = 0;
        components.retain_mut(|x| {
            if let Some((e, c)) = x {
                if !sort_needed {
                    let key = c.key();
                    match last_key {
                        Some(x) if x > key => {
                            sort_needed = true;
                        }
                        _ => {
                            last_key = Some(key);
                            indexes[e.index as usize].index = index;
                            index += 1;
                        }
                    }
                }
                true
            } else {
                false
            }
        });

        if sort_needed {
            components.sort_by_key(|x| x.key());
        }

        sort_needed
    }

    /// 更新する
    pub fn update(&mut self) {
        // ソートが行われたかどうか
        let is_sort_performed = self.update_components();

        if is_sort_performed {
            let mut index = 0;
            for c in self.components.iter_mut() {
                // retain済みなのでunwrap
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
        let is_sort_performed = self.update_components();

        if is_sort_performed {
            let mut index = 0;
            for c in self.components.iter_mut() {
                // retain済みなのでunwrap
                let (entity, comp) = c.as_mut().unwrap();
                f(entity, comp)?;
                self.indexes[entity.index as usize].index = index;
                index += 1;
            }
        } else {
            for c in self.components.iter_mut() {
                // retain済みなのでunwrap
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
