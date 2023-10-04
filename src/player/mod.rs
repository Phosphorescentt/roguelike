use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(FixedUpdate, player_move);
    }
}

#[derive(Component)]
pub struct Player;

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((
            Player,
            MaterialMesh2dBundle {
                mesh: meshes
                    .add(bevy::prelude::shape::Quad::new(Vec2::new(50., 50.)).into())
                    .into(),
                material: materials.add(ColorMaterial::from(Color::ANTIQUE_WHITE)),
                transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                ..default()
            },
        ))
        .insert(Collider::ball(25.))
        .insert(RigidBody::Dynamic)
        .insert(Name::new("Player"))
        .with_children(|p| {
            p.spawn(Camera2dBundle::default());
        });
}

fn player_move(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time_step: Res<FixedTime>,
) {
    let mut player_transform = player_query.single_mut();
    let mut direction = Vec3::new(0., 0., 0.);
    if keyboard_input.pressed(KeyCode::W) {
        direction.x += 0.;
        direction.y += 1.;
    }
    if keyboard_input.pressed(KeyCode::A) {
        direction.x += -1.;
        direction.y += 0.;
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction.x += 0.;
        direction.y += -1.;
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction.x += 1.;
        direction.y += 0.;
    }

    player_transform.translation += direction * 100. * time_step.period.as_secs_f32();
}
