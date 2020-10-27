use ggez::*;
use hecs::*;
use crate::components::*;

pub fn system_rect_draw(world: &mut World, context: &mut Context) {
    for (id, (transform, rect)) in &mut world.query::<(&Transform, &Rect)>() {
        let mut draw_param = graphics::DrawParam::default();
        draw_param.dest = transform.position.into();
        let mesh = graphics::Mesh::new_rectangle(context, graphics::DrawMode::fill(), rect.rect, graphics::WHITE).unwrap();
        graphics::draw(context, &mesh, draw_param).unwrap();
    }
}