use current::*;

fn main() {
    Outliner::run();
}

struct Outliner {}

impl Game for Outliner {
    fn init(_: &mut GameData) -> Self {
        Self {}
    }
}
