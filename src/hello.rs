extern crate bevy;
use bevy::{prelude::{App, Plugin}, ecs::{system::{Resource, Res, ResMut, Query}, query::With}, time::{Timer, Time, TimerMode}, app::{Startup, Update}};

use crate::{types::{Name, Person}, startup};

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_systems(Startup, startup::add_people)
        .add_systems(
            Update,
            (
                greet_people,
            ),
        );
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(
    time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    
    for name in &query {
        println!("hello {}! at time {:?}", name.0, timer.0.tick(time.delta()).duration());
    }
}