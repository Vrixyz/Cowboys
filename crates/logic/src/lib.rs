mod display;
mod input;
mod logic;
mod network;

use bevy::prelude::*;
use bevy_ggrs::*;
use display::DisplayPlugin;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn run() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.53, 0.53, 0.53)))
        .insert_resource(logic::RoundState::NotReady)
        .add_plugins(DefaultPlugins)
        .add_plugin(GGRSPlugin)
        .add_plugin(DisplayPlugin)
        .with_input_system(input::input)
        .add_startup_system(logic::setup)
        .add_startup_system(logic::spawn_players)
        .add_startup_system(network::start_matchbox_socket)
        .add_system(network::wait_for_players)
        .add_system(logic::update_round.exclusive_system())
        .add_system(logic::react_end_round.exclusive_system())
        .add_system(logic::compute_end_round)
        .with_rollback_schedule(Schedule::default().with_stage(
            "ROLLBACK_STAGE",
            SystemStage::single_threaded().with_system(input::move_players),
        ))
        .register_rollback_type::<logic::ActionShield>()
        .register_rollback_type::<logic::ActionReload>()
        .register_rollback_type::<logic::ActionFire>()
        .register_rollback_type::<logic::Health>()
        .register_rollback_type::<logic::Ammunition>()
        .run();
}
