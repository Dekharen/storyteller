use bevy::prelude::*;
use bevy_egui::EguiPlugin;

pub struct EguiLoader;

impl Plugin for EguiLoader {
    fn build(&self, app: &mut App) {
        app.add_plugins((EguiPlugin::default(), super::font_system::UiFontPlugin));
        // .add_plugins(UiFontPlugin);
    }
}
