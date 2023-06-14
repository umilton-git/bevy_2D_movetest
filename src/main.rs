use bevy::prelude::*;
use bevy_asset::AssetServer;
use bevy_ecs::prelude::{Commands, Res};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(keyboard_input_system)
        .run();
}

// My ECS; Wrapper structs make new types of components from existing types; empty components are used as tags
#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerName(String);

#[derive(Component)]
struct PlayerSpeed(u32);

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Bundle)]
struct PlayerBundle {
    name: PlayerName,
    speed: PlayerSpeed,
    _p: Player,

    // We can nest/include another bundle.
    // Add the components for a standard Bevy Sprite:
    #[bundle]
    sprite: SpriteSheetBundle,
}
//

// My Systems

// Main Setup; Spawns Camera and Player Sprite

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    // Now you can do whatever you want with the asset server, such as loading an asset:
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("../assets/sprites/LilDude.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        Direction::None,
    ));
}

/// Moves sprite based on input

fn keyboard_input_system(
    time: Res<Time>,
    mut sprite_position: Query<(&mut Direction, &mut Transform)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Up => transform.translation.y += 150. * time.delta_seconds(),
            Direction::Down => transform.translation.y -= 150. * time.delta_seconds(),
            Direction::Right => transform.translation.x += 150. * time.delta_seconds(),
            Direction::Left => transform.translation.x -= 150. * time.delta_seconds(),
            Direction::None => info!("Not Moving"),
        }

        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            *logo = Direction::Up;
        } else if keyboard_input.just_released(KeyCode::Up)
            || keyboard_input.just_released(KeyCode::W)
        {
            *logo = Direction::None;
        }

        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            *logo = Direction::Down;
        } else if keyboard_input.just_released(KeyCode::Down)
            || keyboard_input.just_released(KeyCode::S)
        {
            *logo = Direction::None;
        }

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            *logo = Direction::Left;
        } else if keyboard_input.just_released(KeyCode::Left)
            || keyboard_input.just_released(KeyCode::A)
        {
            *logo = Direction::None;
        }

        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            *logo = Direction::Right;
        } else if keyboard_input.just_released(KeyCode::Right)
            || keyboard_input.just_released(KeyCode::D)
        {
            *logo = Direction::None;
        }
    }
}
//
