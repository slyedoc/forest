use std::fmt;

use crate::editor::{EditorAction, EditorCameraAction};
use bevy::prelude::*;
use bevy_egui::*;
use bevy_input_actionmap::*;

pub struct ActionsWindow {
    pub enabled: bool,
}
pub fn draw_actions(
    egui_ctx: Res<EguiContext>,
    editor_map: Res<InputMap<EditorAction>>,
    editor_camera_map: Res<InputMap<EditorCameraAction>>,
    mut window: ResMut<ActionsWindow>,
) {
    egui::Window::new("Key Bindings")
        .open(&mut window.enabled)
        .show(egui_ctx.ctx(), |ui| {
            dispaly_input_map::<EditorAction>(&editor_map, ui);
            dispaly_input_map::<EditorCameraAction>(&editor_camera_map, ui);
        });
}

fn dispaly_input_map<T: 'static + Send + Sync + fmt::Display>(
    input_map: &Res<InputMap<T>>,
    ui: &mut egui::Ui,
) {
    for (a, b) in &input_map.actions {
        for binding in &b.bindings {
            for key in &binding.keys {
                ui.horizontal(|ui| {
                    ui.set_enabled(false);
                    let _ = ui.button(format!("{:?}", key));
                    ui.label(format!("{} ", a));
                });
            }
            for key in &binding.mouse_buttons {
                ui.horizontal(|ui| {
                    ui.set_enabled(false);
                    let _ = ui.button(format!("Mouse - {:?}", key));
                    ui.label(format!("{} ", a));
                });
            }

            // TODO: Add other button types, like gamepad
        }
    }
}
