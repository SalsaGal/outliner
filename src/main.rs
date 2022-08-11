use current::sprite::{Sprite, Transform};
use current::*;
use image::{DynamicImage, GenericImageView};
use winit::event::{Event, WindowEvent};

fn main() {
    Outliner::run();
}

struct Outliner {
    original_image: Option<(DynamicImage, Sprite)>,
}

impl Game for Outliner {
    fn init(data: &mut GameData) -> Self {
        data.set_resizable(true);
        data.set_title("Outliner");

        let original_image = match std::env::args().nth(1) {
            Some(path) => {
                let image = image::open(path).unwrap();
                let (width, height) = image.dimensions();
                let id = data.graphics.load_image(&image, sprite::Filter::Nearest);
                let sprite = Sprite::new_texture_rect(data.graphics, id)
                    .with_transform(Transform::scale((width as f32, height as f32).into()));
                Some((image, sprite))
            }
            None => None,
        };

        Self {
            original_image,
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
                    let (width, height) = image.dimensions();
                    let id = data.graphics.load_image(&image, sprite::Filter::Nearest);
                    let sprite = Sprite::new_texture_rect(data.graphics, id)
                        .with_transform(Transform::scale((width as f32, height as f32).into()));
                    self.original_image = Some((image, sprite));
                }
                Err(err) => eprintln!("Error: {err}"),
            }
        }
    }
}
