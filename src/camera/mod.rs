use bevy::prelude::*;

use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0., 0., 0.)));
        // .add_systems(Startup, setup_camera);
        // .add_systems(Update, camera_controller);
    }
}

// pub fn setup_camera(mut commands: Commands, mut player_query: Query<Entity, With<Camera>>) {
//     let player = player_query.single_mut();
//     let camera = commands.spawn(Camera2dBundle::default()).id();
//     // commands
//     //     .entity(player_query.single_mut())
//     //     .push_children(&[camera]);
// }

// Commented out as the camera is now being created as a child of the player meaning it's
// locked to the player's position automatically. Not sure if this will have to change in
// the future as I'd like a nicer/smoother camera controller, but for now this is ideal.
// fn camera_controller(
//     mut camera_query: Query<&mut Transform, With<Camera>>,
//     player_query: Query<&mut Transform, (With<Player>, Without<Camera>)>,
// ) {
//     let player_transform = player_query.single();
//     let mut camera_transform = camera_query.single_mut();
//     camera_transform.translation = player_transform.translation;
// }
