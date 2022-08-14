use current::sprite::{RectTextureMap, Sprite, Transform};
use current::*;
use glam::Vec2;
use image::DynamicImage;
use winit::event::{Event, WindowEvent};

fn main() {
    Outliner::run();
}

struct Outliner {
    background: Sprite,
    original_image: Option<(DynamicImage, Sprite)>,
}

fn get_original_image(data: &mut GameData, path: &str) -> Option<(DynamicImage, Sprite)> {
    match image::open(path) {
        Ok(image) => {
            let position = -(image.width() as f32) / 2.0;
            let id = data.graphics.load_image(&image, sprite::Filter::Nearest);
            let sprite = Sprite::new_texture_rect(data.graphics, id).with_transform(
                Transform::scale((image.width() as f32, image.height() as f32).into())
                    .with_translation((position, 0.0, 0.0).into()),
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

        let background_id = data.graphics.load_image(
            &image::load_from_memory(include_bytes!("background.png")).unwrap(),
            sprite::Filter::Nearest,
        );
        let background_scale = data.graphics.get_frame_size();

        Self {
            background: Sprite::new_texture_rect_mapped(
                data.graphics,
                background_id,
                RectTextureMap::from_grid(Vec2::new(1.0, 1.0) / 16.0, (0.0, 0.0).into()),
            )
            .with_transform(Transform::scale(background_scale).with_z(-1.0)),
            original_image: match std::env::args().nth(1) {
                Some(path) => get_original_image(data, &path),
                None => None,
            },
        }
    }

    fn render<'a>(&'a mut self, mut frame: graphics::Frame<'a>) {
        self.background.render_to(&mut frame);
        if let Some((_, sprite)) = &self.original_image {
            sprite.render_to(&mut frame);
        }
    }

    fn handle_event(&mut self, data: &mut GameData, event: &Event<()>) {
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::DroppedFile(path) => {
                    self.original_image = get_original_image(data, path.to_str().unwrap())
                }
                WindowEvent::Resized(size) => {
                    let scale = Vec2::new(size.width as f32, size.height as f32);
                    self.background
                        .set_transform(Transform::scale(scale).with_z(-1.0));
                }
                _ => {}
            }
        }
    }
}
