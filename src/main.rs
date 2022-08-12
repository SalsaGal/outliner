use current::sprite::{Sprite, Transform};
use current::*;
use image::DynamicImage;
use winit::event::{Event, WindowEvent};

fn main() {
    Outliner::run();
}

struct Outliner {
    original_image: Option<(DynamicImage, Sprite)>,
}

fn get_original_image(data: &mut GameData, path: &str) -> Option<(DynamicImage, Sprite)> {
    match image::open(path) {
        Ok(image) => {
            let id = data.graphics.load_image(&image, sprite::Filter::Nearest);
            let sprite = Sprite::new_texture_rect(data.graphics, id).with_transform(
                Transform::scale((image.width() as f32, image.height() as f32).into())
            );
            Some((image, sprite))
        }
        Err(_) => None,
    }
}

impl Game for Outliner {
    fn init(data: &mut GameData) -> Self {
        data.set_resizable(true);
        data.set_title("Outliner");

        Self {
            original_image: match std::env::args().nth(1) {
                Some(path) => get_original_image(data, &path),
                None => None,
            },
        }
    }

    fn render<'a>(&'a mut self, mut frame: graphics::Frame<'a>) {
        if let Some((_, sprite)) = &self.original_image {
            sprite.render_to(&mut frame);
        }
    }

    fn handle_event(&mut self, data: &mut GameData, event: &Event<()>) {
        if let Event::WindowEvent {
            event: WindowEvent::DroppedFile(path),
            ..
        } = event
        {
            self.original_image = get_original_image(data, path.to_str().unwrap());
        }
    }
}
