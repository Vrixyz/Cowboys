use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use ggrs::P2PSession;

use crate::{
    logic::{ActionFire, ActionReload, ActionShield, Ammunition, Health, Player, RoundState},
    states::GameState,
};

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(GameState::AssetLoading)
            .continue_to_state(GameState::Matchmaking)
            .with_collection::<ImageAssets>()
            .build(app);
        app.insert_resource(TexturesEgui::default());
        app.add_state(GameState::AssetLoading);
        app.add_plugin(EguiPlugin);
        app.add_system(health);
        app.add_system(ammo);
        app.add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(round_time_progress)
                .with_system(actions_display),
        );
    }
}

#[derive(AssetCollection)]
pub struct ImageAssets {
    #[asset(path = "bandit.png")]
    bandit: Handle<Image>,
    #[asset(path = "tumbleweed.png")]
    tumbleweed: Handle<Image>,
    #[asset(path = "gunshot.png")]
    gunshot: Handle<Image>,
    #[asset(path = "shield.png")]
    shield: Handle<Image>,
    #[asset(path = "reload-gun-barrel.png")]
    reload: Handle<Image>,
}

pub struct TexturesEgui {
    gunshot: u64,
    shield: u64,
    reload: u64,
}

impl Default for TexturesEgui {
    fn default() -> Self {
        Self {
            gunshot: 2,
            shield: 3,
            reload: 4,
        }
    }
}

#[derive(Component)]
pub struct DisplayPlayer {
    pub handle: usize,
}
#[derive(Component)]
pub struct DisplayRoundProgress {
    from: Vec2,
    to: Vec2,
}

pub(crate) fn spawn_display_static(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    images: Res<ImageAssets>,
    mut egui_textures: ResMut<TexturesEgui>,
) {
    egui_context.set_egui_texture(egui_textures.gunshot, images.gunshot.clone());
    egui_context.set_egui_texture(egui_textures.shield, images.shield.clone());
    egui_context.set_egui_texture(egui_textures.reload, images.reload.clone());
    // Player 1
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 2., 0.)),
            texture: images.bandit.clone(),
            sprite: Sprite {
                color: Color::rgb(0., 0.47, 1.),
                custom_size: Some(Vec2::new(1., 1.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(DisplayPlayer { handle: 0 });

    // Player 2
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., -2., 0.)),
            texture: images.bandit.clone(),
            sprite: Sprite {
                color: Color::rgb(0., 0.4, 0.),
                custom_size: Some(Vec2::new(1., 1.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(DisplayPlayer { handle: 1 });

    // Round progress
    let mut c = commands.spawn_bundle(SpriteBundle {
        transform: Transform::from_translation(Vec3::new(-2., 0., 0.)),
        texture: images.tumbleweed.clone(),
        sprite: Sprite {
            color: Color::rgb(0.64, 0.79, 0.69),
            custom_size: Some(Vec2::new(1., 1.)),
            ..Default::default()
        },
        ..Default::default()
    });
    c.insert(DisplayRoundProgress {
        from: Vec2::new(-2.0, 0.0),
        to: Vec2::new(2.0, 0.0),
    });
}

fn round_time_progress(
    session: Res<P2PSession>,
    round_state: Res<RoundState>,
    mut hp_query: Query<(&mut Transform, &DisplayRoundProgress)>,
) {
    match &*round_state {
        RoundState::WaitUntil(wait) => {
            let duration = wait.until - wait.from;
            let ratio = (session.current_frame() - wait.from) as f32 / duration as f32;
            for (mut t, def) in hp_query.iter_mut() {
                t.translation = def.from.lerp(def.to, ratio).extend(0.0);
            }
        }
        _ => {}
    }
}

fn health(
    window: Res<Windows>,
    egui_context: Res<EguiContext>,
    mut hp_query: Query<(&Player, &Health)>,
) {
    let win = window.get_primary().expect("no primary window");
    for (i, (_, hp)) in hp_query.iter().enumerate() {
        egui::Window::new(format!("name_{}", i.to_string()))
            .fixed_size((150f32, 50f32))
            .title_bar(false)
            .fixed_pos((i as f32 * 150f32, 0f32))
            .show(egui_context.ctx(), |ui| {
                ui.label(format!("Player {}", i.to_string()));
            });

        egui::Window::new(format!("HP_{}", i.to_string()))
            .fixed_size((150f32, 50f32))
            .title_bar(false)
            .fixed_pos((i as f32 * 150f32, 25f32))
            .show(egui_context.ctx(), |ui| {
                ui.label(format!("hp: {}", hp.amount.to_string()));
            });
    }
}

fn ammo(
    window: Res<Windows>,
    egui_context: Res<EguiContext>,
    egui_textures: Res<TexturesEgui>,
    mut q_ammo: Query<(&Player, &Ammunition)>,
) {
    let win = window.get_primary().expect("no primary window");
    for (i, (_, ammo)) in q_ammo.iter().enumerate() {
        egui::Window::new(format!("AMMO_{}", i.to_string()))
            .fixed_size((150f32, 50f32))
            .title_bar(false)
            .fixed_pos((
                i as f32 * 150f32,
                50f32, //(win.physical_height() / 2) as f32 - 55f32,
            ))
            .show(egui_context.ctx(), |ui| {
                ui.label(format!("ammo: {}", ammo.amount.to_string()));
            });
    }
}

fn actions_display(
    window: Res<Windows>,
    egui_context: Res<EguiContext>,
    egui_textures: Res<TexturesEgui>,
    round_state: Res<RoundState>,
    mut reload_query: Query<(Entity, &ActionReload, &Player)>,
    mut shield_query: Query<(Entity, &ActionShield, &Player)>,
    mut fire_query: Query<(Entity, &ActionFire, &Player)>,
) {
    let is_wait = matches!(*round_state, RoundState::WaitUntil(_));
    if !matches!(*round_state, RoundState::DisplayUntil(_)) && !is_wait {
        return;
    }
    for (i, (e, reload, player)) in reload_query.iter().enumerate() {
        if !reload.is_active {
            continue;
        }
        let title = if is_wait { "Will reload" } else { "Reloading" };
        let image = egui::TextureId::User(egui_textures.reload);
        raw_display_action(title, image, i, &egui_context);
    }
    for (i, (e, shield, player)) in shield_query.iter().enumerate() {
        if !shield.is_active {
            continue;
        }
        raw_display_action(
            if is_wait { "Will shield" } else { "Shielding" },
            egui::TextureId::User(egui_textures.shield),
            i,
            &egui_context,
        );
    }
    for (i, (e, fire, player)) in fire_query.iter().enumerate() {
        if !fire.is_active {
            continue;
        }
        raw_display_action(
            if is_wait { "Will fire" } else { "Firing" },
            egui::TextureId::User(egui_textures.gunshot),
            i,
            &egui_context,
        );
    }
}

fn raw_display_action(
    title: &str,
    image: egui::TextureId,
    player_id: usize,
    egui_context: &Res<EguiContext>,
) {
    egui::Window::new(format!("{title}_{}", player_id.to_string()))
        .fixed_size((150f32, 50f32))
        .title_bar(false)
        .fixed_pos((
            player_id as f32 * 150f32,
            75f32, //(win.physical_height() / 2) as f32 - 55f32,
        ))
        .show(egui_context.ctx(), |ui| {
            ui.label(title);
            ui.image(image, [50.0, 50.0]);
        });
}
