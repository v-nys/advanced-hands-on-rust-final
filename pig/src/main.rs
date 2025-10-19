use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, egui};
use my_library::RandomNumberGenerator;

// Vincent: States is specificially for state machine view of games
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

// Vincent: dit is een "tag" component, bevat zelf geen extra info
// dient voor de dobbelstenen
// elke dobbelsteen die deel uitmaakt van een reeks heeft dit
#[derive(Component)]
struct HandDie;

#[derive(Resource)] // newtype
struct Random(RandomNumberGenerator);

#[derive(Resource)]
// Vincent: dit is omdat we niet meteen elke dobbelsteen tegelijk willen rollen voor CPU
struct HandTimer(Timer);

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d::default());
    let texture = asset_server.load("dice.png");
    // Vincent: 6 vierkantjes met zijden van 52 pixels
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(52), 6, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.insert_resource(GameAssets {
        image: texture,
        layout: texture_atlas_layout,
    });
    commands.insert_resource(Scores { cpu: 0, player: 0 });
    commands.insert_resource(Random(RandomNumberGenerator::new()));
    commands.insert_resource(HandTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
}

fn display_score(scores: Res<Scores>, mut egui_context: EguiContexts) {
    egui::Window::new("Total Scores").show(egui_context.ctx_mut(), |ui| {
        ui.label(&format!("Player: {}", scores.player));
        ui.label(&format!("CPU: {}", scores.cpu));
    });
}

// Vincent: dus we vragen alle dobbelstenen die deel uitmaken van huidige hand?
// we achterhalen hoe de afbeelding er uitziet en we plaatsen ze
// score updatet nog niet, dat is pas aan einde beurt
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

// eigenlijk eerder "clear_dice": alle dobbelstenen uit huidige hand verdwijnen
fn clear_die(hand_query: &Query<(Entity, &Sprite), With<HandDie>>, commands: &mut Commands) {
    hand_query
        .iter()
        .for_each(|(entity, _)| commands.entity(entity).despawn());
}

fn player(
    hand_query: Query<(Entity, &Sprite), With<HandDie>>,
    mut commands: Commands,
    mut rng: ResMut<Random>,
    assets: Res<GameAssets>,
    mut scores: ResMut<Scores>,
    mut state: ResMut<NextState<GamePhase>>,
    mut egui_context: EguiContexts,
) {
    egui::Window::new("Play Options").show(egui_context.ctx_mut(), |ui| {
        // bepaal de *huidige hand* score, niet de huidige totaalscore
        let hand_score: usize = hand_query
            .iter()
            .map(|(_, ts)| ts.texture_atlas.as_ref().unwrap().index + 1)
            .sum();
        ui.label(&format!("Score for this hand: {hand_score}"));
        if ui.button("Roll Dice").clicked() {
            let new_roll = rng.0.range(1..=6);
            if new_roll == 1 {
                clear_die(&hand_query, &mut commands);
                state.set(GamePhase::Cpu);
            } else {
                spawn_die(&hand_query, &mut commands, &assets, new_roll, Color::WHITE);
            }
        }
        if ui.button("Pass - Keep Hand Score").clicked() {
            let hand_total: usize = hand_query
                .iter()
                .map(|(_, ts)| ts.texture_atlas.as_ref().unwrap().index + 1)
                .sum();
            scores.player += hand_total;
            clear_die(&hand_query, &mut commands);
            state.set(GamePhase::Cpu);
        }
    });
}

#[allow(clippy::too_many_arguments)]
fn cpu(
    hand_query: Query<(Entity, &Sprite), With<HandDie>>,
    mut state: ResMut<NextState<GamePhase>>,
    mut scores: ResMut<Scores>,
    mut rng: ResMut<Random>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut timer: ResMut<HandTimer>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        let hand_total: usize = hand_query
            .iter()
            .map(|(_, ts)| ts.texture_atlas.as_ref().unwrap().index + 1)
            .sum();
        // Vincent: CPU mikt dus op 20 of hoger en wil in totaal score van 100 halen
        if hand_total < 20 && scores.cpu + hand_total < 100 {
            let new_roll = rng.0.range(1..=6);
            if new_roll == 1 {
                clear_die(&hand_query, &mut commands);
                state.set(GamePhase::Player);
            } else {
                spawn_die(
                    &hand_query,
                    &mut commands,
                    &assets,
                    new_roll as usize,
                    Color::Srgba(Srgba::new(0.0, 0.0, 1.0, 1.0)),
                );
            }
        } else {
            scores.cpu += hand_total;
            state.set(GamePhase::Player);
            hand_query
                .iter()
                .for_each(|(entity, _)| commands.entity(entity).despawn());
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: false,
        })
        .add_systems(Startup, setup)
        .init_state::<GamePhase>()
        .add_systems(
            Update,
            (
                display_score,
                player.run_if(in_state(GamePhase::Player)),
                cpu.run_if(in_state(GamePhase::Cpu)),
            ),
        )
        .run();
}
