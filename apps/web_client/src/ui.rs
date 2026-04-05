use bevy::prelude::*;
use legion_core::components::King;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui);
        app.add_systems(Update, (update_king_hp, update_wave_counter, update_gold));
    }
}

#[derive(Component)]
struct KingHpBar;

#[derive(Component)]
struct WaveCounterText;

#[derive(Component)]
struct GoldText;

fn setup_ui(mut commands: Commands) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        // Selection HUD
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            ..default()
        }).with_children(|col| {
            col.spawn((
                TextBundle::from_section(
                    "Gold: 0",
                    TextStyle {
                        font_size: 28.0,
                        color: Color::srgb(1.0, 0.8, 0.0), // Gold!
                        ..default()
                    },
                ),
                GoldText,
            ));
            col.spawn(TextBundle::from_section(
                "Phase 4 Beta HUD\n1 - Footman (120g)\n2 - Archer (135g)\n3 - Mage (160g)\nRight Click - Sell / Backspace - Sell All\nSPACE to start Combat Phase!",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });

        // King HP
        parent.spawn((
            TextBundle::from_section(
                "King HP: 1000",
                TextStyle {
                    font_size: 30.0,
                    color: Color::srgb(0.8, 0.2, 0.2),
                    ..default()
                },
            ),
            KingHpBar,
        ));

        // Wave Tracker
        parent.spawn((
            TextBundle::from_section(
                "Enemies Left: 0",
                TextStyle {
                    font_size: 25.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            WaveCounterText,
        ));
    });
}

fn update_king_hp(
    kings: Query<&King>,
    mut text_query: Query<&mut Text, With<KingHpBar>>,
    state: Res<State<legion_core::state::GamePhase>>,
) {
    if state.get() == &legion_core::state::GamePhase::GameOver {
        if let Ok(mut text) = text_query.get_single_mut() {
            text.sections[0].value = "DEFEAT! King is Dead!".to_string();
            text.sections[0].style.color = Color::srgb(1.0, 0.0, 0.0);
            text.sections[0].style.font_size = 50.0;
        }
        return;
    }
    
    if let Ok(king) = kings.get_single() {
        if let Ok(mut text) = text_query.get_single_mut() {
            text.sections[0].value = format!("King HP: {}", king.hp);
        }
    }
}

fn update_wave_counter(
    creeps: Query<&legion_core::components::Creep>,
    mut text_query: Query<&mut Text, With<WaveCounterText>>,
) {
    let count = creeps.iter().count();
    if let Ok(mut text) = text_query.get_single_mut() {
        text.sections[0].value = format!("Enemies Left: {}", count);
    }
}

fn update_gold(
    economy: Query<&legion_core::components::PlayerEconomy>,
    mut text_query: Query<&mut Text, With<GoldText>>,
) {
    if let Ok(econ) = economy.get_single() {
        if let Ok(mut text) = text_query.get_single_mut() {
            text.sections[0].value = format!("Gold: {}", econ.gold);
        }
    }
}
