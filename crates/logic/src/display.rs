use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

use crate::logic::{Ammunition, Health};

pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin);
        app.add_system(health);
        app.add_system(ammo);
    }
}

fn health(
    window: Res<Windows>,
    egui_context: Res<EguiContext>,
    mut hp_query: Query<(&Transform, &Health)>,
) {
    let win = window.get_primary().expect("no primary window");
    for (i, (t, hp)) in hp_query.iter().enumerate() {
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
    mut q_ammo: Query<(&Transform, &Ammunition)>,
) {
    let win = window.get_primary().expect("no primary window");
    for (i, (t, ammo)) in q_ammo.iter().enumerate() {
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
