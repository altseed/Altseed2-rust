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
    alive: bool,
}

impl Component for CameraComponent {
    fn alive(&self) -> bool {
        self.alive
    }

    fn alive_mut(&mut self) -> &mut bool {
        &mut self.alive
    }
}

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
            alive: true,
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
            for (e, drawn) in drawn_storage
                .storage
                .iter()
                .filter(|(_, d)| d.camera_group() & group == group)
            {
                self.drawn_entities.push_with(*e, drawn.z_order());
            }
        } else {
            let mut sort_needed = false;
            self.drawn_entities.retain_mut(|e| {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CameraID {
    entity: Entity,
}

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

    /// CameraComponentのaliveをfalseに指定します。
    /// 削除の反映は次のフレームまで遅延されます。
    pub fn remove(&mut self, id: CameraID) -> bool {
        self.storage.remove(id.entity)
    }

    pub fn push(&mut self, component: CameraComponent) -> CameraID {
        let entity = self.storage.push(component);
        CameraID { entity }
    }
}