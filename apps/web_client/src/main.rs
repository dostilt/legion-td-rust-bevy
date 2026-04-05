use bevy::prelude::*;
use legion_core::LegionCorePlugin;
use avian3d::prelude::*;

mod scene;
mod spawn;
mod movement;
mod vfx;
mod input;
mod phases;
mod ui;
mod combat;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .init_resource::<legion_core::state::GlobalCombatState>()
        // Render colliders in debug for visibility
        // .add_plugins(PhysicsDebugPlugin::default()) 
        .add_plugins(LegionCorePlugin)
        .add_plugins(scene::ScenePlugin)
        .add_plugins(spawn::SpawnPlugin)
        .add_plugins(movement::MovementPlugin)
        .add_plugins(vfx::VfxPlugin)
        .add_plugins(input::InputPlugin)
        .add_plugins(phases::PhasesPlugin)
        .add_plugins(ui::UIPlugin)
        .add_plugins(combat::CombatPlugin)
        .run();
}
