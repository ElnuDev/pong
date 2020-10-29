use ggez::*;

use rapier2d::na::Vector2;
use rapier2d::dynamics::{JointSet, RigidBodySet, IntegrationParameters};
use rapier2d::geometry::{BroadPhase, NarrowPhase, ColliderSet};
use rapier2d::pipeline::PhysicsPipeline;

use crate::settings;

pub struct PhysicsWorld {
    pub physics_pipeline: PhysicsPipeline,
    pub gravity: Vector2<f32>,
    pub integration_parameters: IntegrationParameters,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub joint_set: JointSet,
    // TO-DO: event handler
}

impl PhysicsWorld {
    pub fn new() -> PhysicsWorld {
        PhysicsWorld {
            physics_pipeline: PhysicsPipeline::new(),
            gravity: Vector2::new(0.0, settings::GRAVITY), // 32 pixels is one meter 
            integration_parameters: IntegrationParameters::default(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            joint_set: JointSet::new(),
        }
    }

    pub fn step(&mut self, context: &Context) {
        self.integration_parameters.set_dt(timer::delta(context).as_secs_f32());

        let event_handler = ();
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.joint_set,
            None,
            None,
            &event_handler
        )
    }
}