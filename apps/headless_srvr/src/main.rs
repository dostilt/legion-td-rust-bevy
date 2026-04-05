use bevy::prelude::*;
use legion_core::LegionCorePlugin;
use legion_core::state::GamePhase;
use avian3d::prelude::*;

fn main() {
    println!("Legion TD server initialized");
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(LegionCorePlugin)
        .init_state::<GamePhase>()
        .run();
}

