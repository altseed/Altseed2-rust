use super::{Component, Entity};

/// 順序が保証されないComponent Storage
#[derive(Debug)]
pub struct Storage<T: Component> {
    removed_entities: Vec<Entity>,
    indexes: Vec<Entity>,
    components: Vec<(Entity, T)>,
}

impl<T: Component> Storage<T> {
    pub fn new() -> Self {
        Storage {
            removed_entities: Vec::new(),
            indexes: Vec::new(),
            components: Vec::new(),
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, (Entity, T)> {
        self.components.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, (Entity, T)> {
        self.components.iter_mut()
    }

    pub fn contains(&self, entity: Entity) -> bool {
        match self.indexes.get(entity.index as usize) {
            Some(e) if e.version == entity.version => true,
            _ => false,
        }
    }

    pub fn len(&self) -> usize {
        self.components.len()
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        let e = self.indexes.get(entity.index as usize)?;
        if e.version != entity.version {
            return None;
        }

        Some(&self.components[e.index as usize].1)
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        let e = self.indexes.get(entity.index as usize)?;
        if e.version != entity.version {
            return None;
        }

        Some(&mut self.components[e.index as usize].1)
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

        self.components.push((entity, item));
        entity
    }

    /// 要素を削除します。削除に成功したら格納されていた要素を返します。
    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        let index = {
            let e = self.indexes.get_mut(entity.index as usize)?;

            if e.version != entity.version {
                return None;
            }

            e.version += 1;
            e.index
        };

        let (_, res) = self.components.swap_remove(index as usize);
        if !self.components.is_empty() {
            let swapped = self.components[index as usize].0.index;
            self.indexes[swapped as usize].index = index;
        }

        self.removed_entities.push(Entity {
            version: entity.version + 1,
            ..entity
        });

        Some(res)
    }

    /// 全ての要素を削除します。
    pub fn clear(&mut self) {
        while let Some((e, _)) = self.components.pop() {
            self.removed_entities.push(Entity {
                version: e.version + 1,
                ..e
            });
        }
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

    impl Component for TestC {}

    impl TestC {
        fn new() -> Self {
            TestC {
                key: Memoried::new(0),
            }
        }
    }

    #[test]
    fn add_remove_storage() {
        let mut storage = Storage::new();

        let _e1 = storage.add(TestC::new());
        let e2 = storage.add(TestC::new());
        let _e3 = storage.add(TestC::new());

        assert_eq!(storage.indexes.len(), 3);
        assert_eq!(storage.components.len(), 3);
        assert_eq!(storage.removed_entities.len(), 0);

        storage.remove(e2);
        assert_eq!(storage.indexes.len(), 3);
        assert_eq!(storage.components.len(), 2);
        assert_eq!(storage.removed_entities.len(), 1);

        storage.add(TestC::new());
        assert_eq!(storage.indexes.len(), 3);
        assert_eq!(storage.components.len(), 3);
        assert_eq!(storage.removed_entities.len(), 0);
    }
}
