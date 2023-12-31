use bevy::{
    prelude::*,
};


mod components;
pub use components::*;
mod systems;
pub use systems::*;
mod events;
pub use events::*;

pub struct PluginUI;
impl Plugin for PluginUI {
    fn build(&self, app: &mut App) {
        app.add_event::<GridCursorEvent>();
        app.insert_resource(UI::default());
        app.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
        add_systems(app);
    }
}
