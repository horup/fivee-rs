use bevy::prelude::{App, Entity, Event};

#[derive(Event)]
pub enum GameEvent {
    IsNowActive { entity: Entity },
}

pub fn build(app: &mut App) {
    app.add_event::<GameEvent>();
}
