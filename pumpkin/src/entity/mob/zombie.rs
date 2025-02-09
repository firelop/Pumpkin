use std::sync::Arc;

use pumpkin_data::entity::EntityType;
use pumpkin_util::math::vector3::Vector3;
use uuid::Uuid;

use crate::{
    entity::ai::goal::{look_at_entity::LookAtEntityGoal, target_goal::TargetGoal},
    server::Server,
    world::World,
};

use super::MobEntity;

pub struct Zombie;

impl Zombie {
    pub async fn make(
        server: &Server,
        position: Vector3<f64>,
        world: &Arc<World>,
    ) -> (Arc<MobEntity>, Uuid) {
        let (zombie_entity, uuid) = server
            .add_mob_entity(EntityType::Zombie, position, world)
            .await;
        zombie_entity.goal(LookAtEntityGoal::new(8.0)).await;
        zombie_entity.goal(TargetGoal::new(16.0)).await;
        (zombie_entity, uuid)
    }
}
