use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, egui};
use my_library::RandomNumberGenerator;

// what is States?
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default, States)]
enum GamePhase {
    #[default]
    Player,
    Cpu,
}

#[derive(Resource)]
struct GameAssets {
    image: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

#[derive(Clone, Copy, Resource)]
struct Scores {
    player: usize,
    cpu: usize,
}

#[derive(Component)]
struct HandDie;

#[derive(Resource)] // newtype
struct Random(RandomNumberGenerator);

#[derive(Resource)]
struct HandTimer(Timer);

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d::default());
    let texture = asset_server.load("dice.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(52), 6, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.insert_resource(GameAssets {
        image: texture,
        layout: texture_atlas_layout,
    });
    commands.insert_resource(Scores { cpu: 0, player: 0 });
    commands.insert_resource(Random(RandomNumberGenerator::new()));
    commands.insert_resource(HandTimer(Timer::from_seconds(0.5, TimerMode::Repeating)));
}

fn display_score(scores: Res<Scores>, mut egui_context: EguiContexts) {
    egui::Window::new("Total Scores").show(egui_context.ctx_mut(), |ui| {
        ui.label(&format!("Player: {}", scores.player));
        ui.label(&format!("CPU: {}", scores.cpu));
    });
}

fn spawn_die(
    hand_query: &Query<(Entity, &Sprite), With<HandDie>>,
    commands: &mut Commands,
    assets: &GameAssets,
    new_roll: usize,
    color: Color,
) {
    let rolled_die = hand_query.iter().count() as f32 * 52.0;
    let mut sprite = Sprite::from_atlas_image(
        assets.image.clone(),
        TextureAtlas {
            layout: assets.layout.clone(),
            index: new_roll - 1,
        },
    );
    sprite.color = color;
    commands.spawn((
        sprite,
        Transform::from_xyz(rolled_die - 400.0, 60.0, 1.0),
        HandDie,
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin {
            // ??
            enable_multipass_for_primary_context: false,
        })
        .add_systems(Startup, setup)
        .init_state::<GamePhase>()
        .add_systems(Update, display_score)
        .add_systems(Update, player.run_if(in_state(GamePhase::Player)))
        .add_systems(Update, cpu.run_if(in_state(GamePhase::Cpu)))
        .run();
}
