use ggez::*;

pub struct SpriteComponent {
    pub image: graphics::Image,
}

impl SpriteComponent {
    pub fn new (context: &mut Context, path: &'static str) -> SpriteComponent {
        let mut image = graphics::Image::new(context, path).unwrap();
        image.set_filter(graphics::FilterMode::Nearest);
        SpriteComponent {
            image,
        }
    }
}