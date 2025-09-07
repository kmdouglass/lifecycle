use bevy::prelude::*;

use lifecycle::core::ResourceType;
use lifecycle::components::{Money, Position, Resources};
use lifecycle::resources::OccupiedPositions;

const INIT_MONEY: u32 = 10;
const INIT_RESOURCES: u32 = 1;
const NUM_AGENTS: usize = 10;

fn main() {
    App::new()
        .insert_resource(OccupiedPositions::default())
        .add_systems(Startup, spawn_agents)
        .run();
}

fn spawn_agents(mut commands: Commands, mut occupied: ResMut<OccupiedPositions>) {
    for i in 0..NUM_AGENTS {
        let x = (i % 5) as u32;
        let y = (i / 5) as u32;

        if occupied.occupy(x, y) {
            commands.spawn((
                Money(INIT_MONEY),
                Position { x, y },
                starting_resources()
            ));
        }
        
        
    }
}

fn starting_resources() -> Resources {
    let mut resources = Resources::new();
    resources.add(ResourceType::Food, INIT_RESOURCES);
    resources.add(ResourceType::Wood, INIT_RESOURCES);
    resources.add(ResourceType::Stone, INIT_RESOURCES);
    resources
}