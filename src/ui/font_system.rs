use std::sync::Arc;

use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};

pub struct UiFontPlugin;

impl Plugin for UiFontPlugin {
    fn build(&self, app: &mut App) {
        //TODO: Has to be run after egui plugin is loaded
        app.add_systems(PostStartup, setup_custom_font);
    }
}

fn setup_custom_font(mut contexts: EguiContexts) -> Result {
    // Load your font file (can be .ttf or .otf)
    // TODO: Use the asset server...
    //
    // let font_bytes: Handle<Vec<u8>> = asset_server.load("fonts/your_font.ttf");
    let ctx = contexts.ctx_mut()?;
    // --- OPTION 1: if you want to use the AssetServer loader asynchronously ---
    // You’ll need to defer font setup until the asset is actually loaded.
    // Otherwise, you can use include_bytes!() for a static font below.

    // --- OPTION 2 (simpler): embed font bytes directly ---
    let mono_font_data = include_bytes!("../../assets/fonts/FiraCodeMonoRegular.ttf");
    let font_data = include_bytes!("../../assets/fonts/FiraCodeRegular.ttf");

    let mut fonts = egui::FontDefinitions::default();

    // Insert our custom font under a name
    fonts.font_data.insert(
        "custom_font".to_owned(),
        Arc::new(egui::FontData::from_owned(font_data.to_vec())),
    );
    fonts.font_data.insert(
        "custom_font_mono".to_owned(),
        Arc::new(egui::FontData::from_owned(mono_font_data.to_vec())),
    );

    // Configure font priorities: "Proportional" = normal text, "Monospace" = code/textareas
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "custom_font".to_owned());
    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .insert(0, "custom_font_mono".to_owned());

    // Apply the font definitions to all Egui contexts
    ctx.set_fonts(fonts);

    // Optionally adjust text scaling (especially for HiDPI)
    let mut style = (*ctx.style()).clone();
    style.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(18.0, egui::FontFamily::Proportional),
    );
    ctx.set_style(style);
    Ok(())
}
