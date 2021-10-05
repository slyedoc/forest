use bevy::{
    app::AppExit,
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_egui::{
    egui::{menu, Checkbox, TopBottomPanel},
    EguiContext,
};
use bevy_inspector_egui::{plugin::InspectorWindows, WorldInspectorParams};

pub fn toolbar_system(
    egui_ctx: Res<EguiContext>,
    //mut state: ResMut<State<AppState>>,
    mut exit: EventWriter<AppExit>,
    mut world_inspection: ResMut<WorldInspectorParams>,
    mut inspector_windows: ResMut<InspectorWindows>,
    diagnostics: Res<Diagnostics>,
    mut egui_settings: Local<bool>,
    mut egui_inspection: Local<bool>,
) {
    use bevy_inspector_egui::egui::Window;

    TopBottomPanel::top("top_panel").show(egui_ctx.ctx(), |ui| {
        menu::bar(ui, |ui| {
            menu::menu(ui, "App", |ui| {
                if ui.button("Quit").clicked() {
                    exit.send(AppExit);
                }
            });

            menu::menu(ui, "Windows", |ui| {
                ui.add(Checkbox::new(&mut world_inspection.enabled, "World"));
            });

            menu::menu(ui, "Resources", |ui| {
                for (_, w) in inspector_windows.iter_mut() {
                    ui.add(Checkbox::new(&mut w.visible, &w.name));
                }
            });

            menu::menu(ui, "Egui", |ui| {
                ui.add(Checkbox::new(&mut egui_settings, "Egui Settings"));
                ui.add(Checkbox::new(&mut egui_inspection, "Egui Inspection"));
            });

            // TODO: Figure out better way to align right
            let desired_size = ui.available_width();
            ui.add_space(desired_size - 50.0);

            ui.horizontal(|ui| {
                if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                    if let Some(fps_value) = fps.value() {
                        ui.label(format!("FPS {:.0}", fps_value,));
                    }
                }
            });
        });
    });

    Window::new("Inspection")
        .open(&mut egui_inspection)
        .scroll(true)
        .show(egui_ctx.ctx(), |ui| {
            egui_ctx.ctx().inspection_ui(ui);
        });

    Window::new("Settings")
        .open(&mut egui_settings)
        .scroll(true)
        .show(egui_ctx.ctx(), |ui| {
            egui_ctx.ctx().settings_ui(ui);
        });
}

pub fn close_windows_system(mut inspector_windows: ResMut<InspectorWindows>) {
    for (_, w) in inspector_windows.iter_mut() {
        w.visible = false;
    }
}
