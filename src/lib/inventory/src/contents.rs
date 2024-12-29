use crate::slot::Slot;
use dashmap::DashMap;

pub struct InventoryContents {
    pub contents: DashMap<i32, Slot>,
}

impl InventoryContents {
    pub fn empty() -> Self {
        Self {
            contents: DashMap::new(),
        }
    }

    pub fn set_slot(&mut self, slot_id: i32, slot: Slot) -> &mut Self {
        self.contents.insert(slot_id, slot);
        self
    }

    pub fn get_slot(&self, item: i32) -> Option<Slot> {
        self.contents.get(&item).map(|v| *v)
    }
}
