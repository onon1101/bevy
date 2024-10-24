use bevy::prelude::*;
use bevy::prelude::CursorIcon::Default;
use bevy::window::PrimaryWindow;

const PLAYER_SPEED: f32 = 500.0f32;
const PLAYER_SIZE: f32 = 64.0f32;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_camera)
        .add_system(player_movement)
        .add_system(confine_player_movement)
        .run();

}

#[derive(Component)]
pub struct Player {}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(
        (
            SpriteBundle{
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                texture: asset_server.load("sprites/ball_blue_large.png"),
                ..default()
            },
            Player{},
        )
    );
}

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window: &Window = window_query.get_single().unwrap();

    commands.spawn(
        (
            Camera2dBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                ..default()
            }
            )
    );
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>
) {
    let mut transfrom = player_query.get_single_mut().unwrap();

    transfrom.translation += {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        direction * PLAYER_SPEED * time.delta_seconds()
    };
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&mut Window, With<PrimaryWindow>>
) {
    if let Ok(mut player_transform) = player_query.get_single_mut()  {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;

        player_transform.translation = {
            let (x_min, x_max, y_min, y_max) = (
                0.0 + half_player_size,
                window.width() - half_player_size,
                0.0 + half_player_size,
                window.height() - half_player_size,
            );

            let mut translation: Vec3 = player_transform.translation;

            translation.x =
                if translation.x < x_min { x_min }
                else if translation.x > x_max { x_max }
                else { translation.x };


            translation.y =
                if translation.y < y_min { y_min }
                else if translation.y > y_max { y_max } 
                else { translation.y };

            translation
        };
    }



}