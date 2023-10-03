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
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_systems(Startup, setup);

    // Add debug stuff
    #[cfg(feature = "debug")]
    app.add_plugin(RapierDebugRenderPlugin::default());

    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(bevy::prelude::shape::Quad::new(Vec2::new(50., 50.)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::DARK_GRAY)),
            transform: Transform::from_translation(Vec3::new(100., 100., 0.)),
            ..default()
        })
        .insert(Collider::cuboid(25., 25.))
        .insert(RigidBody::Fixed);
}
