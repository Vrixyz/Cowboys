use crate::logic::RoundState;

use super::logic::ActionFire;
use super::logic::ActionReload;
use super::logic::ActionShield;
use super::logic::Player;
use bevy::prelude::*;

pub const INPUT_SIZE: usize = std::mem::size_of::<u8>();

const INPUT_RELOAD: u8 = 1 << 0;
const INPUT_SHIELD: u8 = 1 << 1;
const INPUT_FIRE: u8 = 1 << 2;

pub(crate) fn handle_inputs(
    mut round_state: ResMut<RoundState>,
    inputs: Res<Vec<ggrs::GameInput>>,
    mut player_query: Query<(
        &mut ActionReload,
        &mut ActionShield,
        &mut ActionFire,
        &Player,
    )>,
) {
    if !matches!(round_state.as_ref(), RoundState::WaitUntil(_)) {
        return;
    }
    for (mut reload, mut shield, mut fire, player) in player_query.iter_mut() {
        let input = inputs[player.handle].buffer[0];

        if input & INPUT_RELOAD != 0 {
            reload.is_active = false;
            shield.is_active = false;
            fire.is_active = false;

            reload.is_active = true;
            continue;
        }
        if input & INPUT_SHIELD != 0 {
            reload.is_active = false;
            shield.is_active = false;
            fire.is_active = false;

            shield.is_active = true;
            continue;
        }
        if input & INPUT_FIRE != 0 {
            reload.is_active = false;
            shield.is_active = false;
            fire.is_active = false;

            fire.is_active = true;
            continue;
        }
    }
}

pub(crate) fn local_input(_: In<ggrs::PlayerHandle>, keys: Res<Input<KeyCode>>) -> Vec<u8> {
    let mut input = 0u8;
    if keys.any_just_pressed([KeyCode::R]) {
        input |= INPUT_RELOAD;
    }
    if keys.any_just_pressed([KeyCode::S]) {
        input |= INPUT_SHIELD;
    }
    if keys.any_just_pressed([KeyCode::F]) {
        input |= INPUT_FIRE
    }
    vec![input]
}
