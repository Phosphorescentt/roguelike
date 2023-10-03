use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_hanabi::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct Player;

const SPEED: f32 = 100.;

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
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (player_move, camera_controller));

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
    commands.spawn(Camera2dBundle::default());

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
        .insert(RigidBody::Dynamic);
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

    player_transform.translation += direction * SPEED * time_step.period.as_secs_f32();
}

fn camera_controller(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&mut Transform, (With<Player>, Without<Camera>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation = player_transform.translation;
}
