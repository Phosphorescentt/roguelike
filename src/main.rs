mod camera;
mod magic;
mod player;
mod world;

use bevy::prelude::*;
use bevy::render::settings::{WgpuFeatures, WgpuSettings};
use bevy_hanabi::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

use crate::camera::CameraPlugin;
use crate::magic::MagicPlugin;
use crate::player::PlayerPlugin;
use crate::world::WorldPlugin;

fn main() {
    let mut wgpu_settings = WgpuSettings::default();
    wgpu_settings
        .features
        .set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);

    let mut app = App::new();

    // Set gravity to 0 in bevy_rapier2d. (This can probably get moved to another
    // plugin at some point when I add all the configuration infrastructure)
    app.insert_resource(RapierConfiguration {
        gravity: Vect::Y * 0.,
        ..default()
    });

    app.add_plugins(DefaultPlugins)
        .add_plugins(HanabiPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.));

    // Load my plugins
    app.add_plugins(CameraPlugin)
        .add_plugins(MagicPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(WorldPlugin);

    // Add debug stuff
    #[cfg(feature = "debug")]
    app.add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::default());

    app.run();
}
