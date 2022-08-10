use current::*;
use image::DynamicImage;
use winit::event::{Event, WindowEvent};

fn main() {
    Outliner::run();
}

struct Outliner {
    original_image: Option<DynamicImage>,
}

impl Game for Outliner {
    fn init(data: &mut GameData) -> Self {
        data.set_resizable(true);
        data.set_title("Outliner");

        Self {
            original_image: None,
        }
    }

    fn handle_event(&mut self, _: &mut GameData, event: &Event<()>) {
        if let Event::WindowEvent { event: WindowEvent::DroppedFile(path), .. }  = event {
            match image::open(path) {
                Ok(image) => {
                    self.original_image = Some(image);
                },
                Err(err) => eprintln!("Error: {err}"),
            }
        }
    }
}
