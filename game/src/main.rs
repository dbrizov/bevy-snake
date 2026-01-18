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
        .add_systems(Update, print_fps)
        .run();
}

fn print_fps(time: Res<Time>) {
    println!("{}", 1.0 / time.delta_secs());
}
