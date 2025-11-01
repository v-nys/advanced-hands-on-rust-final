use bevy::prelude::*;
use my_library::*;

#[derive(Clone, PartialEq, Eq, Debug, Hash, Default, States, Copy)]
enum GamePhase {
    #[default]
    MainMenu,
    Flapping,
    GameOver,
}

// Vincent: having this component means there is upward/downward acceleration
#[derive(Component)]
struct Flappy {
    //(1)
    gravity: f32, //(2)
}

#[derive(Component)]
struct FlappyElement;

// Vincent: tag struct?
#[derive(Component)]
struct Obstacle; //(3)

fn main() -> anyhow::Result<()> {
    let mut app = App::new();
    add_phase!(app, GamePhase, GamePhase::Flapping, start => [setup], run => [gravity, flap, clamp, move_walls, hit_wall], exit => [cleanup::<FlappyElement>]);
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            //(5)
            title: "Flappy Dragon - Bevy Edition".to_string(),
            resolution: bevy::window::WindowResolution::new(1024.0, 768.0),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(RandomPlugin)
    .add_plugins(GameStatePlugin::new(
        GamePhase::MainMenu,
        GamePhase::Flapping,
        GamePhase::GameOver,
    ))
    .add_plugins(
        AssetManager::new()
            .add_image("dragon", "flappy_dragon.png")?
            .add_image("wall", "wall.png")?,
    )
    .run();
    Ok(())
}

fn setup(
    mut commands: Commands,
    rng: ResMut<RandomNumberGenerator>,
    assets: Res<AssetStore>,
    loaded_assets: AssetResource,
) {
    commands.spawn((Camera2d::default(), FlappyElement));
    spawn_image!(
        assets,
        commands,
        "dragon",
        -490.0,
        0.0,
        1.0,
        &loaded_assets,
        Flappy { gravity: 0.0 },
        FlappyElement
    );
    build_wall(&mut commands, &assets, &loaded_assets, rng.range(-5..5));
}

fn build_wall(
    commands: &mut Commands,
    assets: &AssetStore,
    loaded_assets: &LoadedAssets,
    gap_y: i32,
) {
    for y in -12..=12 {
        if y < gap_y - 4 || y > gap_y + 4 {
            spawn_image!(
                assets,
                commands,
                "wall",
                512.0,
                y as f32 * 32.0,
                1.0,
                &loaded_assets,
                Obstacle,
                FlappyElement
            );
        }
    }
}

fn gravity(mut query: Query<(&mut Flappy, &mut Transform)>) {
    if let Ok((mut flappy, mut transform)) = query.single_mut() {
        //(16)
        flappy.gravity += 0.1; //(17)
        transform.translation.y -= flappy.gravity; //(18)
    }
}

fn flap(keyboard: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Flappy>) {
    if keyboard.pressed(KeyCode::Space) {
        if let Ok(mut flappy) = query.single_mut() {
            flappy.gravity = -5.0; //(19)
        }
    }
}

fn clamp(mut query: Query<&mut Transform, With<Flappy>>, mut state: ResMut<NextState<GamePhase>>) {
    if let Ok(mut transform) = query.single_mut() {
        if transform.translation.y > 384.0 {
            transform.translation.y = 384.0;
        } else if transform.translation.y < -384.0 {
            state.set(GamePhase::GameOver);
        }
    }
}

fn move_walls(
    mut commands: Commands,
    mut query: Query<&mut Transform, With<Obstacle>>,
    delete: Query<Entity, With<Obstacle>>,
    assets: Res<AssetStore>,
    loaded_assets: AssetResource,
    rng: ResMut<RandomNumberGenerator>,
) {
    let mut rebuild = false;
    for mut transform in query.iter_mut() {
        transform.translation.x -= 4.0;
        if transform.translation.x < -530.0 {
            rebuild = true; //(23)
        }
    }
    if rebuild {
        for entity in delete.iter() {
            commands.entity(entity).despawn();
        }
        build_wall(&mut commands, &assets, &&loaded_assets, rng.range(-5..5));
    }
}

fn hit_wall(
    player: Query<&Transform, With<Flappy>>,
    walls: Query<&Transform, With<Obstacle>>,
    mut state: ResMut<NextState<GamePhase>>,
) {
    if let Ok(player) = player.single() {
        //(26)
        for wall in walls.iter() {
            //(27)
            let distance = player.translation.distance(wall.translation); //(28)
            if distance < 32.0 {
                state.set(GamePhase::GameOver);
            }
        }
    }
}
