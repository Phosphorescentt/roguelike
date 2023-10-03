use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world);
    }
}

fn setup_world(
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
        .insert(RigidBody::Fixed)
        .insert(Name::new("DARK_GRAY square"));
}
