use bevy::{app::{App, Plugin, Update}, ecs::system::Res, time::Time};

pub struct UnitFactoryPlugin;

impl Plugin for UnitFactoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (create_unit));
    }
}

fn create_unit(time: Res<Time>) {
    
}