//! Effectful non-reusable feature modules go here. Things like a lobby or a world (or main_world and lobby_world etc etc) or the ui/hud connected to the game data.
//! Primary semantic of these modules is that each one is a single 'feature' of the game.

pub mod lobby;
pub mod multiplayer;
pub mod music;
pub mod ui;
