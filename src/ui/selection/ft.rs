use crate::{
    ExecutableConfiguration, FileTypeSelection,
    ui::{
        components::{separator, ui_flex_spacer},
        style::*,
    },
    visualization::DroppedFile,
};
use bevy_egui::egui::{self, Color32};
use egui_taffy::{
    Tui, TuiBuilderLogic,
    bg::simple::{TuiBackground, TuiBuilderLogicWithBackground},
};

pub fn ui_executable_options(
    tui: &mut Tui,
    dropped: &DroppedFile,
    cfg: &mut ExecutableConfiguration,
) {
    tui.style(compose_style([column(), full_size(), gap_y(16.)]))
        .bg_add(
            TuiBackground::new()
                .with_background_color(Color32::BLUE)
                .with_corner_radius(5.),
            |tui| {
                tui.ui(|ui| {
                    ui.toggle_value(
                        &mut cfg.use_interpreter,
                        egui::RichText::new("Enable Interpreter").size(40.),
                    );
                });
                tui.style(compose_style([row()])).add(|tui| {
                    if !cfg.use_interpreter {
                        cfg.interpreter = None;
                    }

                    if cfg.use_interpreter {
                        tui.ui(|ui| {
                            ui.label(
                                egui::RichText::new("Interpreter Type :")
                                    .size(32.)
                                    .underline(),
                            );
                        });
                        tui.ui(|ui| {
                            egui::ComboBox::from_id_salt("INTERPRETER_TYPE_SELECTOR")
                                .selected_text(
                                    egui::RichText::new(
                                        cfg.interpreter
                                            .as_ref()
                                            .map_or("No interpreter selected...", |itp| &itp.0),
                                    )
                                    .size(32.),
                                )
                                .show_ui(ui, |ui| {
                                    FileTypeSelection::show_interpreters(ui, cfg);
                                });
                        });
                    }
                });

                tui.ui(separator);
                ui_flex_spacer(tui);
                // check(ui, &mut cfg.check);
                tui.style(compose_style([flex(), align_self_center()]))
                    .ui(|ui| {
                        ui.code(
                            egui::RichText::new(format!(
                                "Executable command : {}{}",
                                cfg.interpreter
                                    .as_ref()
                                    .map_or(String::new(), |itp| itp.to_display_string()),
                                dropped.0.file_name().unwrap_or_default().display(),
                            ))
                            .size(22.),
                        );
                    });
            },
        );
    // });
}
