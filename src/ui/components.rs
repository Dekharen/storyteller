use bevy_egui::egui;
use egui_taffy::{Tui, TuiBuilderLogic, taffy::Style};
// use egui_taffy::{Tui, TuiBuilderLogic, taffy::prelude::*};

pub fn padded_button(
    ui: &mut egui::Ui,
    button: egui::Button,
    padding: egui::Vec2,
) -> egui::Response {
    // let ui = ui.egui_ui_mut();
    ui.scope(|ui| {
        ui.style_mut().spacing.button_padding = padding;
        ui.add(button)
    })
    .inner
}

pub fn separator(ui: &mut egui::Ui) {
    // let ui = ui.egui_ui_mut();
    ui.scope(|ui| {
        // let widgets = &mut ui.visuals_mut().widgets;
        ui.visuals_mut().widgets.noninteractive.bg_stroke =
            egui::Stroke::new(3.0, egui::Color32::DARK_GRAY);
        ui.add(egui::Separator::default().spacing(14.));
    });
}

// fn ui_separator(tui: &mut Tui) {
//     tui.ui(|ui| separator(ui));
// }
//
pub fn ui_flex_spacer(tui: &mut Tui) {
    tui.style(Style {
        flex_grow: 1.0,
        ..Default::default()
    })
    .add(|_| {});
}
