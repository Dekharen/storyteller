use crate::{FileTypeSelection, ui::style::*};
use bevy_egui::egui;
use egui_taffy::{Tui, TuiBuilderLogic};

pub fn ui_file_type_selector(tui: &mut Tui, selection: &mut FileTypeSelection) {
    tui.style(compose_style([row()])).add(|tui| {
        tui.label(egui::RichText::new("File Type :").size(38.));
        tui.ui(|ui| {
            egui::ComboBox::from_id_salt("FILETYPE_SELECTOR")
                .selected_text(egui::RichText::new(selection.to_text()).size(38.))
                .show_ui(ui, |ui| {
                    FileTypeSelection::show_selectables(ui, selection);
                });
        });
    });
}
