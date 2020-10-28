use rapier2d::*;

use ggez::*;
use crate::physics_world::*;

pub fn system_physics(physics_world: &mut PhysicsWorld, context: &Context) {
    physics_world.step(context);
}