use bevy::prelude::AppExtStates;
use bevy::{ecs::world::FromWorld, state::state::FreelyMutableState, state::state::States};

pub struct GameStatePlugin<T> {
    menu_state: T,
    game_start_state: T,
    game_end_state: T,
}

impl<T> GameStatePlugin<T> {
    #[allow(clippy::new_without_default)]
    pub fn new(menu_state: T, game_start_state: T, game_end_state: T) -> Self {
        Self {
            menu_state,
            game_start_state,
            game_end_state,
        }
    }
}

impl<T: States + FromWorld + FreelyMutableState> bevy::prelude::Plugin for GameStatePlugin<T> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_state::<T>();
    }
}
