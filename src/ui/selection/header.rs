use crate::{
    FileTypeSelection,
    file_id::FileTypeSuggestion,
    ui::{components::padded_button, style::*},
    visualization::{DroppedFile, VisualizerState},
};
use bevy::{ecs::system::Commands, state::commands::CommandsStatesExt};
use bevy_egui::egui::{self, Color32};
use egui_taffy::{
    AsTuiBuilder, Tui, TuiBuilderLogic,
    bg::simple::{TuiBackground, TuiBuilderLogicWithBackground},
    tui,
};

pub fn ui_selection_header(tui: &mut Tui, commands: &mut Commands) {
    tui.style(compose_style([row(), align_end()])).bg_add(
        TuiBackground::new().with_background_color(Color32::from_hex("#550000").unwrap()),
        |tui: &mut Tui| {
            tui.wrap_mode(egui::TextWrapMode::Extend).ui(|ui| {
                let back = egui::Button::new(egui::RichText::new("← Back").size(32.).strong())
                    .fill(egui::Color32::DARK_RED);

                if padded_button(ui, back, egui::Vec2::new(25., 12.)).clicked() {
                    commands.remove_resource::<FileTypeSuggestion>();
                    commands.remove_resource::<DroppedFile>();
                    commands.remove_resource::<FileTypeSelection>();
                    commands.set_state(VisualizerState::Input);
                }
            });
        },
    );
}
