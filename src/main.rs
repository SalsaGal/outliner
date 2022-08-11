use current::sprite::{Sprite, Transform};
use current::*;
use image::{DynamicImage, GenericImageView};
use winit::event::{Event, WindowEvent};

fn main() {
    Outliner::run();
}

struct Outliner {
    original_image: Option<(DynamicImage, Sprite)>,
    out_image: Option<(DynamicImage, Sprite)>,
}

impl Game for Outliner {
    fn init(data: &mut GameData) -> Self {
        data.set_resizable(true);
        data.set_title("Outliner");
        data.graphics.frame_size = Some((2.0, 2.0).into());

        let original_image = match std::env::args().nth(1) {
            Some(path) => {
                let image = image::open(path).unwrap();
                let id = data.graphics.load_image(&image, sprite::Filter::Nearest);
                let sprite = Sprite::new_texture_rect(data.graphics, id).with_transform(
                    Transform::scale((1.0, 1.0).into()).with_translation((-0.5, 0.0, 0.0).into())
                );
                Some((image, sprite))
            }
            None => None,
        };

        Self {
            original_image,
            out_image: None,
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
            match image::open(path) {
                Ok(image) => {
                    let id = data.graphics.load_image(&image, sprite::Filter::Nearest);
                    let sprite = Sprite::new_texture_rect(data.graphics, id);
                    self.original_image = Some((image, sprite));
                }
                Err(err) => eprintln!("Error: {err}"),
            }
        }
    }
}
