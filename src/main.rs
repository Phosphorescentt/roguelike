mod camera;
mod magic;
mod player;
mod world;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_hanabi::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::camera::CameraPlugin;
use crate::player::PlayerPlugin;
use crate::world::WorldPlugin;

fn main() {
    let mut app = App::new();

    // Add not debug stuff
    app.insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(RapierConfiguration {
            gravity: Vect::Y * 0.,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(HanabiPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.));

    // Add debug stuff
    #[cfg(feature = "debug")]
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.run();
}
