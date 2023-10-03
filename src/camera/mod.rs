use bevy::prelude::*;

use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
            .add_systems(Startup, setup_camera)
            .add_systems(Update, camera_controller);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn camera_controller(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    player_query: Query<&mut Transform, (With<Player>, Without<Camera>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation = player_transform.translation;
}
