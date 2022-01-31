mod display;
mod input;
mod logic;
mod network;
mod states;

use bevy::prelude::*;
use bevy_ggrs::*;
use display::*;
use logic::*;
use network::*;
use states::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn run() {
    let mut app = App::new()
        .insert_resource(ClearColor(Color::rgb(0.53, 0.53, 0.53)))
        .insert_resource(logic::RoundState::NotReady)
        .insert_resource(bevy::ecs::schedule::ReportExecutionOrderAmbiguities)
        .add_plugins(DefaultPlugins)
        .add_plugin(GGRSPlugin)
        .add_plugin(DisplayPlugin)
        .add_system_set(
            SystemSet::on_enter(GameState::Matchmaking)
                .with_system(start_matchbox_socket)
                .with_system(setup),
        )
        .add_system_set(SystemSet::on_update(GameState::Matchmaking).with_system(wait_for_players))
        .add_system_set(
            SystemSet::on_enter(GameState::InGame)
                .with_system(spawn_players)
                .with_system(spawn_display_static),
        )
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(logic::update_round.exclusive_system())
                .with_system(logic::react_end_round.exclusive_system())
                .with_system(logic::compute_end_round),
        )
        /*    .add_startup_system(network::start_matchbox_socket)
            .add_startup_system(logic::setup)
            .add_system(network::wait_for_players)
            .add_startup_system(logic::spawn_players)*/
        /*    .add_system(logic::update_round.exclusive_system())
            .add_system(logic::react_end_round.exclusive_system())
            .add_system(logic::compute_end_round)
        */
        .with_input_system(input::local_input)
        .with_rollback_schedule(Schedule::default().with_stage(
            "ROLLBACK_STAGE",
            SystemStage::single_threaded().with_system(input::handle_inputs),
        ))
        .register_rollback_type::<logic::ActionShield>()
        .register_rollback_type::<logic::ActionReload>()
        .register_rollback_type::<logic::ActionFire>()
        .register_rollback_type::<logic::Health>()
        .register_rollback_type::<logic::Ammunition>()
        // TODO: add RoundState to rollback types, but it's a State so be careful with that
        .run();
}
