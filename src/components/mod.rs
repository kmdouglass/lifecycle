use bevy::prelude::*;

use crate::core::ResourceType;

#[derive(Component)]
pub struct Money(pub u32);

#[derive(Component)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

#[derive(Component)]
pub struct Resources {
    item: std::collections::HashMap<ResourceType, u32>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            item: std::collections::HashMap::new(),
        }
    }

    pub fn add(&mut self, resource: ResourceType, amount: u32) {
        *self.item.entry(resource).or_insert(0) += amount;
    }

    pub fn get(&self, resource: ResourceType) -> u32 {
        *self.item.get(&resource).unwrap_or(&0)
    }

    pub fn remove(&mut self, resource: ResourceType, amount: u32) -> bool {
        if let Some(current_amount) = self.item.get_mut(&resource) {
            if *current_amount >= amount {
                *current_amount -= amount;
                return true;
            }
        }
        false
    }
}