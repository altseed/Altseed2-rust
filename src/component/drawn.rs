use crate::auto_generated_core_binding::{Graphics, Renderer};

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
    pub(crate) z_order: Memoried<i32>,
    pub(crate) camera_group: Memoried<u32>,
}

impl Component for DrawnComponent {}

impl Sortable<i32> for DrawnComponent {
    fn key(&self) -> &Memoried<i32> {
        &self.z_order
    }

    fn key_mut(&mut self) -> &mut Memoried<i32> {
        &mut self.z_order
    }
}

use crate::error::*;

impl DrawnComponent {
    pub(crate) fn new(kind: DrawnKind) -> Self {
        DrawnComponent {
            kind,
            is_drawn: true,
            z_order: Memoried::new(0),
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
        self.z_order.value()
    }

    pub fn z_order_mut(&mut self) -> &mut i32 {
        self.z_order.value_mut()
    }

    pub fn camera_group(&self) -> u32 {
        self.camera_group.value()
    }

    pub fn camera_group_mut(&mut self) -> &mut u32 {
        self.camera_group.value_mut()
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

    pub(crate) fn on_drawing(&mut self, entity: Entity, camera_storage: &mut CameraStorage) {
        if self.camera_group.is_updated() {
            for container in camera_storage.storage.iter_mut() {
                // 生存していなかったらskip
                // カメラのグループが更新されていたらカメラ側でまとめて取り出すので追加しない。
                if !container.alive || !container.data.key().is_updated() {
                    continue;
                }

                let group = container.data.group();

                if self.camera_group() & group == group {
                    container.data.add_drawn(entity, self.z_order());
                }
            }
        }
    }

    pub(crate) fn draw(
        &mut self,
        _graphics: &mut Graphics,
        renderer: &mut Renderer,
    ) -> AltseedResult<()> {
        if self.is_drawn {
            match &mut self.kind {
                DrawnKind::Sprite(x) => {
                    x.update_transform();
                    renderer.draw_sprite(&mut x.instance);
                }
                DrawnKind::Text(x) => {
                    x.update_transform();
                    renderer.draw_text(&mut x.instance)
                }
                DrawnKind::Polygon(x) => {
                    x.update_transform();
                    renderer.draw_polygon(&mut x.instance)
                }
            }
        }

        Ok(())
    }

    // Memoriedなfieldの更新
    pub(crate) fn update_memoried(&mut self) {
        self.z_order.update();
        self.camera_group.update();
    }
}

/// DrawnComponentにアクセスするためのID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DrawnID {
    entity: Entity,
}

/// DrawnComponentを格納する
#[derive(Debug)]
pub struct DrawnStorage {
    pub(crate) storage: SortedStorage<DrawnComponent, i32>,
}

impl DrawnStorage {
    pub(crate) fn new() -> Self {
        DrawnStorage {
            storage: SortedStorage::new(),
        }
    }

    pub fn get(&self, id: DrawnID) -> Option<&DrawnComponent> {
        self.storage.get(id.entity)
    }

    pub fn get_mut(&mut self, id: DrawnID) -> Option<&mut DrawnComponent> {
        self.storage.get_mut(id.entity)
    }

    /// 即座に新しいDrawnComponentを追加します。
    pub fn add(&mut self, component: DrawnComponent) -> DrawnID {
        let entity = self.storage.add(component);
        DrawnID { entity }
    }

    /// 即座に要素を削除します。
    pub fn remove(&mut self, id: DrawnID) -> bool {
        self.storage.remove(id.entity)
    }

    /// 即座に全ての要素を削除します。
    pub fn clear(&mut self) {
        self.storage.clear();
    }
}
