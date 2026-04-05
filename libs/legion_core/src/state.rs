use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GamePhase {
    #[default]
    Preparation,
    Combat,
    Resolution,
    GameOver,
}

#[derive(Resource, Default)]
pub struct GlobalCombatState {
    pub engaged: bool,
}
