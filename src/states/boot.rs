use amethyst::{SimpleState, GameData, StateData, SimpleTrans, Trans};
use amethyst::assets::{ProgressCounter, Completion};

struct BootState {
    progress: ProgressCounter,
}

impl BootState {
    /*pub fn new() -> Self {
        Self {
            progress: ProgressCounter::new(),
        }
    }*/
}

impl SimpleState for BootState {
    fn on_start(&mut self, _data: StateData<GameData>) {
        //data.world.insert(SpriteResource::new(&data.world, &mut self.progress));
    }

    fn update(&mut self, _data: &mut StateData<GameData>) -> SimpleTrans {
        match self.progress.complete() {
            Completion::Loading => Trans::None,
            Completion::Complete => {
                Trans::None
                //Trans::Switch(Box::new(MainState::new()))
            },
            Completion::Failed => Trans::Quit,
        }
    }
}