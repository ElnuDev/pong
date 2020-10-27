use ggez::*;
use hecs::*;
use crate::components::*;

pub fn system_physics (world: &mut World, context: &mut Context) {
    let delta_time = timer::delta(context).as_secs_f32();

    for (id, (transform, rect, physics)) in &mut world.query::<(&mut Transform, &Rect, &mut Physics)>() {
        transform.position.x += physics.velocity.x * delta_time;
        transform.position.y += physics.velocity.y * delta_time;
    }
}