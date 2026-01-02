use crate::{FileTypeSelection, ui::style::*, visualization::DroppedFile};
use bevy::prelude::*;
use bevy_egui::{
    EguiContexts,
    egui::{self, Color32},
};
use egui_taffy::{
    TuiBuilderLogic,
    bg::simple::{TuiBackground, TuiBuilderLogicWithBackground},
    taffy::{self, Style},
    tui,
};
mod ft;
mod header;
mod selector;
use ft::*;
use header::*;
use selector::*;
pub fn ui_selection_menu(
    mut commands: Commands,
    dropped: Res<DroppedFile>,
    mut selection: ResMut<FileTypeSelection>,
    mut ctx: EguiContexts,
) -> Result {
    let ctx = ctx.ctx_mut()?;

    // let frame = egui::Frame {
    //     outer_margin: egui::Margin::symmetric(50, 50),
    //     inner_margin: egui::Margin::symmetric(100, 50),
    //     corner_radius: egui::CornerRadius::same(5),
    //     fill: egui::Color32::BLACK,
    //     stroke: egui::Stroke::new(2.0, egui::Color32::from_rgb(10, 10, 30)),
    //     ..Default::default()
    // };

    ctx.style_mut(|style| {
        style.wrap_mode = Some(egui::TextWrapMode::Extend);
    });
    egui::CentralPanel::default().show(ctx, |ui| {
        tui(ui, ui.id().with("selection_root"))
            .reserve_available_space()
            .style(compose_style([full_size(), flex()]))
            .show(|tui| {
                tui.style(compose_style([
                    size_percent(1., 0.95),
                    even_margin_len(10.),
                    flex(),
                ]))
                .bg_add(
                    TuiBackground::new()
                        .with_background_color(Color32::BLACK)
                        .with_corner_radius(5.),
                    |tui| {
                        tui.style(compose_style([
                            column(),
                            size_percent(1., 0.95),
                            gap_y(24.),
                            // even_margin_percent(0.02),
                            even_margin_len(10.),
                            // even_padding_percent(0.02),
                        ]))
                        .add(|tui| {
                            ui_selection_header(tui, &mut commands);

                            ui_file_type_selector(tui, &mut selection);
                            match &mut *selection {
                                FileTypeSelection::Executable(_, cfg) => {
                                    // tui.add(|tui| {
                                    // tui.ui(|ui| {
                                    ui_executable_options(tui, &dropped, cfg);
                                    // });
                                    // });
                                }
                                FileTypeSelection::Text(_) => {
                                    tui.add(|tui| {
                                        tui.ui(|ui| {
                                            ui.code(
                                                egui::RichText::new("Reading text file at :")
                                                    .size(22.),
                                            );
                                            ui.code(
                                                egui::RichText::new(
                                                    dropped.0.to_str().unwrap_or(""),
                                                )
                                                .size(22.),
                                            );
                                        });
                                    });
                                }
                                _ => {}
                            }
                        });
                    },
                );
            });
    });

    Ok(())
}
