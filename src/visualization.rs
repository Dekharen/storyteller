use bevy::prelude::*;
use storyframe::{Renderer, core::configuration::Configuration, engine::VisualizationEngine};

#[derive(Debug)]
pub enum VisualizationKind {
    // Heatmap,
    Grid,
    // Volume,
}

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum VisualizerState {
    #[default]
    Input,
    Loaded,
    Grid,
}

#[derive(Message)]
pub struct LoadVisualization(VisualizationKind);

#[derive(Deref, Resource)]
pub struct VisualizationSettings<T>(T);

#[derive(Deref, Resource)]
pub struct Engine(VisualizationEngine);

#[derive(Component)]
pub struct TaggedEntity;

pub fn configure_visualization_system(
    mut events: MessageReader<LoadVisualization>,
    mut commands: Commands,
    engine: Res<Engine>,
) {
    for message in events.read() {
        trace!(
            "Message received : {:?}. Inserting configuration.",
            message.0
        );
        if let Ok(info) = engine.current_part() {
            commands.insert_resource(VisualizationSettings(info.configuration.clone()));
        };
    }
}
pub fn file_drop(mut commands: Commands, mut evr_dnd: MessageReader<FileDragAndDrop>) {
    // commands.spawn(Text::new("May your woes be many, and your days few..."));
    for ev in evr_dnd.read() {
        if let FileDragAndDrop::DroppedFile { window, path_buf } = ev {
            println!(
                "Dropped file with path: {:?}, in window id: {:?}",
                path_buf, window
            );
        }
    }
}

pub fn load_visualization_system(
    mut events: MessageReader<LoadVisualization>,
    mut next_state: ResMut<NextState<VisualizerState>>,
) {
    //TODO: Impl From/Into kind <=> state
    for message in events.read() {
        let state = match message.0 {
            // VisualizationKind::Heatmap => HeatmapVis::spawn(commands, settings),
            VisualizationKind::Grid => VisualizerState::Grid,
            // VisualizationKind::Volume => VolumeVis::spawn(commands, settings),
        };
        info!("Dispatching state : {state:?}");
        next_state.set(state);
    }
}

pub fn unload_visualization_system(
    mut commands: Commands,
    query: Query<Entity, With<TaggedEntity>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    info!("Exited visualization state");
}

pub struct SimpleGridContext;

storyframe::impl_render_context!(SimpleGridContext => SimpleGridContextTag);

#[derive(Clone)]
pub struct SimpleGrid;

impl SimpleGrid {
    fn spawn(commands: &mut Commands, settings: &VisualizationSettings<Configuration>) {}
}

impl Renderer for SimpleGrid {
    type StateSnapshot = storyframe::domains::text::state::TextSnapshot;
    type Context<'a> = SimpleGridContext;

    fn render_state(&mut self, snapshot: &Self::StateSnapshot, context: &mut Self::Context<'_>) {
        todo!()
    }

    fn renderer_name(&self) -> storyframe::core::id::RendererId {
        "Simple Grid values Renderer"
    }
}
