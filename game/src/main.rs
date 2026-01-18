mod libs;

use crate::libs::bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};
use bevy::prelude::*;

const FPS_TARGET: f64 = 60.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FramepacePlugin)
        .insert_resource(FramepaceSettings {
            limiter: Limiter::from_framerate(FPS_TARGET),
        })
        .add_plugins(HelloPlugin)
        .run();
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people);
        app.add_systems(Update, greet_people);
    }
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Denis Rizov".to_string())));
}

fn greet_people(time: Res<Time>, query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("{}, {:?}", 1.0 / time.delta_secs(), name.0);
    }
}
