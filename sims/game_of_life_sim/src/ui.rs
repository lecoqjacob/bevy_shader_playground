use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Ui},
    EguiContexts, EguiPlugin,
};
use bevy_fn_plugin::bevy_plugin;

use crate::input::AutomataParams;
use crate::SIM_SIZE;

const SPACING: f32 = 10.0;
const TEXT_SIZE: f32 = 15.0;
const HEADING_SIZE: f32 = 20.0;

#[bevy_plugin]
pub fn UIPlugin(app: &mut App) {
    app.add_plugin(EguiPlugin).add_system(user_interface);
}

/// Give our text a custom size
fn sized_text(ui: &mut Ui, text: impl Into<String>) {
    ui.label(
        egui::RichText::new(text)
            .size(TEXT_SIZE)
            .color(egui::Color32::WHITE),
    );
}

fn heading(ui: &mut Ui, text: impl Into<String>) {
    ui.heading(
        egui::RichText::new(text)
            .underline()
            .size(HEADING_SIZE)
            .color(egui::Color32::WHITE),
    );
}

/// System to generate user interface with egui
pub fn user_interface(
    mut contexts: EguiContexts,
    diagnostics: Res<Diagnostics>,
    mut params: ResMut<AutomataParams>,
) {
    egui::Window::new("Automata")
        .constrain(true)
        .fixed_pos(egui::pos2(10.0, 10.0))
        .show(contexts.ctx_mut(), |ui| {
            ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);

            heading(ui, "Info");
            ui.add_space(SPACING);

            if let Some(diag) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(avg) = diag.average() {
                    sized_text(ui, format!("FPS: {:.2}", avg));
                }
            }

            sized_text(ui, format!("Grid size: ({},{})", SIM_SIZE.0, SIM_SIZE.1));

            sized_text(
                ui,
                (if params.is_paused {
                    "Paused"
                } else {
                    "Playing"
                })
                .to_string(),
            );

            ui.add_space(SPACING);
            heading(ui, "Settings");
            ui.add_space(SPACING);

            ui.checkbox(&mut params.use_square_brush, "Square Brush");
            ui.add(egui::Slider::new(&mut params.radius, 0.5..=200.0).text("Brush Size"));
        });
}
