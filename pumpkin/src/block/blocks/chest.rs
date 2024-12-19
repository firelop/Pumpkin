use async_trait::async_trait;
use pumpkin_core::math::{position::WorldPosition};
use pumpkin_inventory::{Chest, OpenContainer, WindowType};
use pumpkin_macros::{pumpkin_block, sound};
use pumpkin_world::{
    block::block_registry::{get_block, get_state_by_properties},
    item::item_registry::Item,
};
use std::collections::HashMap;

use crate::{
    block::{block_manager::BlockActionResult, pumpkin_block::PumpkinBlock},
    entity::player::Player,
    server::Server,
};

#[pumpkin_block("minecraft:chest")]
pub struct ChestBlock;

#[async_trait]
impl PumpkinBlock for ChestBlock {
    async fn on_use<'a>(&self, player: &Player, _location: WorldPosition, server: &Server) {
        self.open_chest_block(player, _location, server).await;
        player
            .world()
            .play_block_sound(sound!("block.chest.open"), _location)
            .await;
    }

    /*
    async fn on_placed<'a>(&self, player: &Player, _location: WorldPosition, _server: &Server) {
        let yaw = player.living_entity.entity.yaw.load();
        let east = WorldPosition(_location.0.add(&Vector3{x: 1, y: 0, z: 0}));
        let west = WorldPosition(_location.0.add(&Vector3{x: -1, y: 0, z: 0}));
        let north = WorldPosition(_location.0.add(&Vector3{x: 0, y: 0, z: 1}));
        let south = WorldPosition(_location.0.add(&Vector3{x: 0, y: 0, z: -1}));

        let state_id;
        if yaw <= -135.0 || yaw >= 135.0 {
            state_id = get_state_by_properties(185, &vec!["south".to_string(), "single".to_string(), "false".to_string()]).unwrap().id;
            if player.world().get_block_state_id(east).await.unwrap() == state_id{
                player.world().set_block_state(_location, get_state_by_properties(185, &vec!["south".to_string(), "right".to_string(), "false".to_string()]).unwrap().id).await;
                player.world().set_block_state(east, get_state_by_properties(185, &vec!["south".to_string(), "left".to_string(), "false".to_string()]).unwrap().id).await;
            }
            else if player.world().get_block_state_id(west).await.unwrap() == state_id {
                player.world().set_block_state(_location, get_state_by_properties(185, &vec!["south".to_string(), "left".to_string(), "false".to_string()]).unwrap().id).await;
                player.world().set_block_state(west, get_state_by_properties(185, &vec!["south".to_string(), "right".to_string(), "false".to_string()]).unwrap().id).await;
            }
            else{
                player.world().set_block_state(_location, state_id).await;
            }

        }
        else if yaw >= -45.0 && yaw <= 45.0 {
            state_id = get_state_by_properties(185, &vec!["north".to_string(), "single".to_string(), "false".to_string()]).unwrap().id;
            if player.world().get_block_state_id(east).await.unwrap() == state_id{
                player.world().set_block_state(_location, get_state_by_properties(185, &vec!["north".to_string(), "left".to_string(), "false".to_string()]).unwrap().id).await;
                player.world().set_block_state(east, get_state_by_properties(185, &vec!["north".to_string(), "right".to_string(), "false".to_string()]).unwrap().id).await;
            }
            else if player.world().get_block_state_id(west).await.unwrap() == state_id {
                player.world().set_block_state(_location, get_state_by_properties(185, &vec!["north".to_string(), "right".to_string(), "false".to_string()]).unwrap().id).await;
                player.world().set_block_state(west, get_state_by_properties(185, &vec!["north".to_string(), "left".to_string(), "false".to_string()]).unwrap().id).await;
            }
            else{
                player.world().set_block_state(_location, state_id).await;
            }
        }
        else if yaw > -135.0 && yaw < -45.0 {
            state_id = get_state_by_properties(185, &vec!["west".to_string(), "single".to_string(), "false".to_string()]).unwrap().id;
            if player.world().get_block_state_id(north).await.unwrap() == state_id{
                player.world().set_block_state(_location, get_state_by_properties(185, &vec!["west".to_string(), "right".to_string(), "false".to_string()]).unwrap().id).await;
                player.world().set_block_state(north, get_state_by_properties(185, &vec!["west".to_string(), "left".to_string(), "false".to_string()]).unwrap().id).await;
            }
            else if player.world().get_block_state_id(south).await.unwrap() == state_id {
                player.world().set_block_state(_location, get_state_by_properties(185, &vec!["west".to_string(), "left".to_string(), "false".to_string()]).unwrap().id).await;
                player.world().set_block_state(south, get_state_by_properties(185, &vec!["west".to_string(), "right".to_string(), "false".to_string()]).unwrap().id).await;
            }
            else {
                player.world().set_block_state(_location, state_id).await;
            }

        }
        else {
            state_id = get_state_by_properties(185, &vec!["east".to_string(), "single".to_string(), "false".to_string()]).unwrap().id;
            if player.world().get_block_state_id(north).await.unwrap() == state_id{
                player.world().set_block_state(_location, get_state_by_properties(185, &vec!["east".to_string(), "left".to_string(), "false".to_string()]).unwrap().id).await;
                player.world().set_block_state(north, get_state_by_properties(185, &vec!["east".to_string(), "right".to_string(), "false".to_string()]).unwrap().id).await;
            }
            else if player.world().get_block_state_id(south).await.unwrap() == state_id {
                player.world().set_block_state(_location, get_state_by_properties(185, &vec!["east".to_string(), "right".to_string(), "false".to_string()]).unwrap().id).await;
                player.world().set_block_state(south, get_state_by_properties(185, &vec!["east".to_string(), "left".to_string(), "false".to_string()]).unwrap().id).await;
            }
            else {
                player.world().set_block_state(_location, state_id).await;
            }
        }
    }
     */

    async fn on_placed<'a>(&self, player: &Player, _location: WorldPosition, _server: &Server) {
        let mut properties: HashMap<&str, &str> = HashMap::new();
        properties.insert("facing", "south");
        properties.insert("type", "single");
        properties.insert("waterlogged", "false");

        player
            .world()
            .set_block_state(
                _location,
                get_state_by_properties(185, &properties).unwrap().id,
            )
            .await;
    }

    async fn on_use_with_item<'a>(
        &self,
        player: &Player,
        _location: WorldPosition,
        _item: &Item,
        server: &Server,
    ) -> BlockActionResult {
        self.open_chest_block(player, _location, server).await;
        BlockActionResult::Consume
    }

    async fn on_close<'a>(&self, player: &Player, _location: WorldPosition, _server: &Server) {
        player
            .world()
            .play_block_sound(sound!("block.chest.close"), _location)
            .await;
    }
}

impl ChestBlock {
    pub async fn open_chest_block(
        &self,
        player: &Player,
        location: WorldPosition,
        server: &Server,
    ) {
        let entity_id = player.entity_id();
        // TODO: This should be a unique identifier for the each block container
        player.open_container.store(Some(2));
        {
            let mut open_containers = server.open_containers.write().await;
            if let Some(chest) = open_containers.get_mut(&2) {
                chest.add_player(entity_id);
            } else {
                let open_container = OpenContainer::new_empty_container::<Chest>(
                    entity_id,
                    Some(location),
                    get_block("minecraft:chest").cloned(),
                );
                open_containers.insert(2, open_container);
            }
        }
        player.open_container(server, WindowType::Generic9x3).await;
    }
}
