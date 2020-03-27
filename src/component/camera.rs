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
    fn key(&self) -> &Memoried<u32> {
        &self.group
    }

    fn key_mut(&mut self) -> &mut Memoried<u32> {
        &mut self.group
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
        drawn_storage: &mut DrawnStorage,
        graphics: &mut Graphics,
        renderer: &mut Renderer,
    ) -> AltseedResult<()> {
        let group = self.group();

        // 自身のカメラグループが更新されているかどうか
        if self.group.is_updated() {
            self.drawn_entities.clear();

            // DrawnComponentはソート済みのはずなので
            for (e, d) in drawn_storage
                .storage
                .iter()
                .filter(|(_, d)| d.camera_group() & group == group)
            {
                self.drawn_entities.push_with(e, d.z_order());
            }
        } else {
            let mut sort_needed = false;
            self.drawn_entities.retain_mut(|e| {
                // 生存しており、カメラグループが合致しているものだけを取り出す。
                if let Some(d) = drawn_storage.storage.get_mut(*e) {
                    sort_needed = d.key().is_updated();
                    d.camera_group() & group == group
                } else {
                    false
                }
            });

            if sort_needed || self.drawn_entities.sort_needed() {
                self.drawn_entities
                    .sort_by_key(|e| drawn_storage.storage.get(*e).unwrap().key().value());
            }
        }

        self.group.update();

        renderer.set_camera(&mut self.instance);

        for e in self.drawn_entities.iter() {
            let d = drawn_storage.storage.get_mut(*e).unwrap();
            d.draw(graphics, renderer)?;
        }

        Ok(())
    }
}

/// CameraComponentにアクセスするためのID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CameraID {
    entity: Entity,
}

/// CameraComponentを格納します。
#[derive(Debug)]
pub struct CameraStorage {
    pub(crate) storage: SortedStorage<CameraComponent, u32>,
}

impl CameraStorage {
    pub(crate) fn new() -> Self {
        CameraStorage {
            storage: SortedStorage::new(),
        }
    }

    pub fn get(&self, id: CameraID) -> Option<&CameraComponent> {
        self.storage.get(id.entity)
    }

    pub fn get_mut(&mut self, id: CameraID) -> Option<&mut CameraComponent> {
        self.storage.get_mut(id.entity)
    }

    /// 即座に新しいCameraComponentを追加します。
    pub fn add(&mut self, component: CameraComponent) -> CameraID {
        let entity = self.storage.add(component);
        CameraID { entity }
    }

    /// 即座に要素を削除します。
    pub fn remove(&mut self, id: CameraID) -> bool {
        self.storage.remove(id.entity)
    }

    /// 即座に全ての要素を削除します。
    pub fn clear(&mut self) {
        self.storage.clear();
    }
}
