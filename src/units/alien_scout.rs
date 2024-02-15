use bevy::{app::{App, Plugin, Update}, ecs::system::{Query, Res}, time::Time, transform::components::Transform};

use crate::structs::{Alien, AlienUnit};

pub struct AlienScout;

impl Plugin for AlienScout {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (run_aliens))  ; 
    }
}

fn run_aliens(mut aliens: Query<(&mut AlienUnit, &mut Transform)>, time: Res<Time>) {
    for (mut alien, transform) in &mut aliens {
        
        alien.health -= 1. * time.delta_seconds();
        if alien.health <= 0. {
            println!("enemy unit is out of health");
            continue;
        }
    }
}