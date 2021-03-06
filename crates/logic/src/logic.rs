use bevy::prelude::*;
use bevy_ggrs::Rollback;
use bevy_ggrs::RollbackIdProvider;
use ggrs::P2PSession;

use ggrs::Frame;

#[derive(Component)]
pub(crate) struct Player {
    pub(crate) handle: usize,
}

#[derive(Component, Default, Reflect)]
pub(crate) struct ActionReload {
    pub(crate) is_active: bool,
}

#[derive(Component, Default, Reflect)]
pub(crate) struct ActionShield {
    pub(crate) is_active: bool,
}

#[derive(Component, Default, Reflect)]
pub(crate) struct ActionFire {
    pub(crate) is_active: bool,
}

#[derive(Component, Default, Reflect)]
pub(crate) struct Health {
    pub(crate) amount: i32,
}

#[derive(Component, Default, Reflect)]
pub(crate) struct Ammunition {
    pub(crate) amount: i32,
}

#[derive(Clone, Copy)]
pub struct RoundWait {
    pub from: Frame,
    pub until: Frame,
}

#[derive(Clone, Copy)]
pub(crate) enum RoundState {
    NotReady,
    WaitUntil(RoundWait),
    DisplayUntil(RoundWait),
    Compute,
    NextRound,
}

pub(crate) struct ComputeRoundResult;

pub(crate) fn setup(mut commands: Commands) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale = 1. / 50.;
    commands.spawn_bundle(camera_bundle);
}

pub(crate) fn spawn_players(mut commands: Commands, mut rip: ResMut<RollbackIdProvider>) {
    // Player 1
    commands
        .spawn()
        .insert(Player { handle: 0 })
        .insert(Rollback::new(rip.next_id()))
        .insert(ActionFire::default())
        .insert(ActionReload { is_active: true })
        .insert(ActionShield::default())
        .insert(Health { amount: 3 })
        .insert(Ammunition::default());

    // Player 2
    commands
        .spawn()
        .insert(Player { handle: 1 })
        .insert(Rollback::new(rip.next_id()))
        .insert(ActionFire::default())
        .insert(ActionReload { is_active: true })
        .insert(ActionShield::default())
        .insert(Health { amount: 3 })
        .insert(Ammunition::default());
}

pub(crate) fn update_round(world: &mut World) {
    if world.get_resource::<P2PSession>().is_some() {
        let frame = world.get_resource::<P2PSession>().unwrap().current_frame();
        dbg!(frame);
        if let Some(mut round_state) = world.get_resource_mut::<RoundState>() {
            if matches!(*round_state, RoundState::NotReady) {
                *round_state = RoundState::WaitUntil(RoundWait {
                    from: frame,
                    until: frame + 60 * 2,
                })
            }
            *round_state = match *round_state {
                RoundState::NotReady => RoundState::WaitUntil(RoundWait {
                    from: frame,
                    until: frame + 60 * 2,
                }),
                RoundState::WaitUntil(wait) => {
                    if wait.until <= frame {
                        info!("displayUntil");
                        RoundState::DisplayUntil(RoundWait {
                            from: frame,
                            until: frame + 60 * 1,
                        })
                    } else {
                        *round_state
                    }
                }
                RoundState::DisplayUntil(wait) => {
                    if wait.until <= frame {
                        info!("round compute");
                        RoundState::Compute
                    } else {
                        *round_state
                    }
                }
                _ => *round_state,
            }
        }
    } else if let Some(mut round_state) = world.get_resource_mut::<RoundState>() {
        if !matches!(*round_state, RoundState::NotReady) {
            *round_state = RoundState::NotReady;
        }
    }
}

pub(crate) fn compute_end_round(
    mut commands: Commands,
    mut round_state: ResMut<RoundState>,
    mut reload_query: Query<(Entity, &mut ActionReload, &Player)>,
    mut shield_query: Query<(Entity, &ActionShield, &Player)>,
    mut fire_query: Query<(Entity, &ActionFire, &Player)>,
    mut ammo_query: Query<&mut Ammunition>,
    mut hp_query: Query<(Entity, &mut Health, &Player)>,
) {
    if matches!(*round_state, RoundState::Compute) {
        let mut vulnerables = vec![0, 1];
        for (e, reload, player) in reload_query.iter() {
            if reload.is_active {
                ammo_query.get_mut(e).unwrap().amount += 1;
            } else {
                vulnerables.retain(|h| *h != player.handle);
            }
        }
        let mut damages = vec![];
        for (e, fire, player) in fire_query.iter() {
            if !fire.is_active {
                continue;
            }
            let mut ammo = ammo_query.get_mut(e).unwrap();
            if ammo.amount <= 0 {
                continue;
            }
            ammo.amount -= 1;
            info!("{} fires, now at {} ammo", player.handle, ammo.amount);
            let attacked_player = if player.handle == 0 { 1 } else { 0 };
            if vulnerables.contains(&attacked_player) {
                damages.push(attacked_player);
            }
        }
        for (e, mut health, player) in hp_query.iter_mut() {
            if damages.contains(&player.handle) {
                info!("{} loses hp, now at {} HP", player.handle, health.amount);
                health.amount -= 1;
                if health.amount <= 0 {
                    // TODO: trigger lose for player!
                    //commands.entity(e).despawn();
                }
            }
        }
        *round_state = RoundState::NextRound;
    }
}

pub(crate) fn react_end_round(world: &mut World) {
    if world.get_resource::<P2PSession>().is_some() {
        let frame = world.get_resource::<P2PSession>().unwrap().current_frame();
        if let Some(mut round_state) = world.get_resource_mut::<RoundState>() {
            if matches!(*round_state, RoundState::NextRound) {
                *round_state = RoundState::WaitUntil(RoundWait {
                    from: frame,
                    until: frame + 60 * 2,
                });
                info!("round wait");
            }
        }
    }
}
