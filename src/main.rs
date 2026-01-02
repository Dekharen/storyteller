use bevy::camera::Viewport;
use bevy::camera::visibility::RenderLayers;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use egui_taffy::taffy::prelude::{AlignItems, FlexDirection, JustifyContent};
use egui_taffy::taffy::{self, prelude::*};
// use bevy::hierarchy::DespawnRecursiveExt;
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::egui::{Align, Color32, ComboBox, Layout, RichText};
// use bevy::render::camera::Camera3d;
// use bevy::render::view::{ComputedVisibility, Visibility};
use bevy_egui::{
    EguiContext, EguiContexts, EguiGlobalSettings, EguiPlugin, EguiPrimaryContextPass,
    PrimaryEguiContext, egui,
};
mod config;
mod file_id;
mod ui;
mod viewports;
mod visualization;
use egui_taffy::{Tui, TuiBuilderLogic, TuiBuilderParams, tui};
use visualization::{
    Engine, LoadVisualization, TaggedEntity, VisualizerState, file_drop, load_visualization_system,
    unload_visualization_system,
};

use crate::file_id::FileTypeSuggestion;
use crate::ui::components::{padded_button, separator};
use crate::ui::selection::ui_selection_menu;
use crate::viewports::{UiSize, ViewportChanged, ViewportId, Viewports};
use crate::visualization::{DroppedFile, HoveredFile};

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
enum UiStatus {
    #[default]
    Visible,
    Invisible,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum VisualizationSystemSet {
    Unload,
    Load,
}

#[derive(Resource)]
struct TickTimer(Timer);
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Storyteller Example".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ui::egui_loader::EguiLoader)
        .init_state::<VisualizerState>()
        .init_state::<UiStatus>()
        .insert_resource(Viewports::default())
        .insert_resource(UiSize::default())
        .insert_resource(TickTimer(Timer::from_seconds(0.01, TimerMode::Repeating)))
        .add_message::<LoadVisualization>()
        .add_message::<ViewportChanged>()
        .configure_sets(
            Update,
            (
                VisualizationSystemSet::Unload,
                VisualizationSystemSet::Load.after(VisualizationSystemSet::Unload),
            ),
        )
        .add_systems(
            Update,
            (
                unload_visualization_system.in_set(VisualizationSystemSet::Unload),
                load_visualization_system.in_set(VisualizationSystemSet::Load),
            )
                .run_if(on_message::<LoadVisualization>),
        )
        // --- Grid ---
        .add_systems(Startup, setup_orbiting_camera)
        .add_systems(Update, camera_orbit_controls)
        .add_systems(OnEnter(VisualizerState::Grid), setup_grid)
        // .add_systems(OnExit(AppState::Grid), cleanup_grid)
        // ^ This will be run in unload_vis anyway
        .add_systems(Update, tick_system.run_if(in_state(VisualizerState::Grid)))
        .add_systems(Update, file_drop.run_if(in_state(VisualizerState::Input)))
        .add_systems(
            Update,
            process_suggestion.run_if(
                not(resource_exists::<FileTypeSelection>)
                    .and(resource_exists::<FileTypeSuggestion>),
            ),
        )
        // .add_systems(
        //     EguiPrimaryContextPass,
        //     ui_selection_menu.run_if(in_state(VisualizerState::Loading)),
        // )
        .add_systems(OnEnter(UiStatus::Invisible), cleanup_ui)
        // --- UI ---
        // .add_systems(
        //     EguiPrimaryContextPass,
        //     (ui_dnd.run_if(not(resource_exists::<Engine>)))
        //         .run_if(in_state(VisualizerState::Input)),
        // )
        .add_systems(
            EguiPrimaryContextPass,
            (
                ui_system.run_if(in_state(UiStatus::Visible)),
                (ui_dnd.run_if(not(resource_exists::<Engine>)))
                    .run_if(in_state(VisualizerState::Input)),
                ui_selection_menu.run_if(
                    in_state(VisualizerState::Loading).and(resource_exists::<FileTypeSelection>),
                ),
            )
                .chain(),
        )
        // --- Input toggle ---
        .add_systems(Update, handle_input)
        .run();
}

fn cleanup_ui(
    mut camera: Single<&mut Camera, Without<EguiContext>>,
    window: Single<&mut Window, With<PrimaryWindow>>,
) {
    let pos = UVec2::new(0, 0);
    let size = UVec2::new(window.physical_width(), window.physical_height());
    camera.viewport = Some(Viewport {
        physical_position: pos,
        physical_size: size,
        ..default()
    });
}
fn setup_orbiting_camera(
    mut commands: Commands,
    mut egui_global_settings: ResMut<EguiGlobalSettings>,
) {
    egui_global_settings.auto_create_primary_context = false;
    commands.spawn((
        Camera3d::default(),
        ViewportId::Primary,
        Transform::from_xyz(5.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
        Visibility::default(),
        OrbitCamera {
            radius: 10.,
            pitch: 0.,
            yaw: 0.,
            target: Vec3::default(),
        },
    ));

    commands.spawn((
        PrimaryEguiContext,
        Camera2d,
        RenderLayers::none(),
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
    ));
}

/// Setup 3D scene
fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Light
    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
        TaggedEntity,
    ));

    // Cubes
    let mesh_handle = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::linear_rgb(0.3, 0.7, 1.0),
        metallic: 0.2,
        ..default()
    });

    for x in -3..=3 {
        for z in -3..=3 {
            commands.spawn((
                Mesh3d(mesh_handle.clone()),
                MeshMaterial3d(material_handle.clone()),
                Transform::from_translation(Vec3::new(x as f32 * 1.5, 0.0, z as f32 * 1.5)),
                GlobalTransform::default(),
                Visibility::default(),
                // ComputedVisibility::default(),
                AnimatedCube,
                TaggedEntity,
            ));
        }
    }
    info!("Entered Grid state");
}

#[derive(Component)]
struct AnimatedCube;

#[derive(Component)]
struct OrbitCamera {
    radius: f32,
    yaw: f32,
    pitch: f32,
    target: Vec3,
}

fn camera_orbit_controls(
    mouse: Res<ButtonInput<MouseButton>>,
    mut motion: MessageReader<MouseMotion>,
    mut scroll: MessageReader<MouseWheel>,
    mut query: Query<(&mut Transform, &mut OrbitCamera), With<Camera>>,
) -> Result {
    let (mut transform, mut orbit) = query.single_mut()?;

    // Handle input
    let (delta_yaw, delta_pitch) = read_mouse_motion(&mut motion, &mouse);
    orbit.yaw += delta_yaw;
    orbit.pitch = (orbit.pitch + delta_pitch).clamp(-1.5, 1.5);

    // Handle zoom
    orbit.radius = adjust_zoom(orbit.radius, &mut scroll);

    // Handle pan
    if mouse.pressed(MouseButton::Middle) {
        orbit.target += compute_pan_vector(&mut motion, transform.rotation);
    }

    // Apply the actual transform update
    apply_orbit_transform(&mut transform, &orbit);
    Ok(())
}

fn read_mouse_motion(
    motion: &mut MessageReader<MouseMotion>,
    mouse: &ButtonInput<MouseButton>,
) -> (f32, f32) {
    if !mouse.pressed(MouseButton::Left) {
        return (0.0, 0.0);
    }

    let mut delta = Vec2::ZERO;
    for ev in motion.read() {
        delta += ev.delta;
    }

    let sensitivity = 0.005;
    (-delta.x * sensitivity, -delta.y * sensitivity)
}

fn adjust_zoom(radius: f32, scroll: &mut MessageReader<MouseWheel>) -> f32 {
    let mut new_radius = radius;
    for ev in scroll.read() {
        new_radius -= ev.y * 0.3;
    }
    new_radius.clamp(1.0, 20.0)
}

fn compute_pan_vector(motion: &mut MessageReader<MouseMotion>, rotation: Quat) -> Vec3 {
    let mut delta = Vec2::ZERO;
    for ev in motion.read() {
        delta += ev.delta;
    }
    let sensitivity = 0.05;
    let right = rotation * Vec3::X;
    let up = rotation * Vec3::Y;
    (-right * delta.x * sensitivity) + (up * delta.y * sensitivity)
}

fn apply_orbit_transform(transform: &mut Transform, orbit: &OrbitCamera) {
    let rotation = Quat::from_rotation_y(orbit.yaw) * Quat::from_rotation_x(orbit.pitch);
    let offset = rotation * Vec3::new(0.0, 0.0, orbit.radius);
    transform.translation = orbit.target + offset;
    transform.look_at(orbit.target, Vec3::Y);
}
/// Fixed tick system (0.5 s)
fn tick_system(
    time: Res<Time>,
    mut timer: ResMut<TickTimer>,
    mut q: Query<&mut Transform, With<AnimatedCube>>, //     mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        // for mesh in material_handles
        // fn random_scale_system() {
        let t = time.elapsed_secs();
        for mut transform in &mut q {
            // use entity position as a seed to vary the result

            let base_height = 1.0;
            let scale_y = 0.5
                + (t + transform.translation.x + transform.translation.z)
                    .sin()
                    .abs(); // example animation
            transform.scale.y = scale_y;
            transform.translation.y = (base_height * scale_y) / 2.0;
        }
    }
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct ExecutableInterpreter(String);
impl ExecutableInterpreter {
    fn to_display_string(&self) -> String {
        String::from(&self.0) + " "
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct ExecutableConfiguration {
    interpreter: Option<ExecutableInterpreter>,
    use_interpreter: bool,
    check: bool,
}

//TODO: Use const values to have each of the &static strings, so they always match :)
#[derive(Resource, Clone, PartialEq, Eq, Debug)]
pub enum FileTypeSelection {
    Directory(&'static str),
    Executable(&'static str, ExecutableConfiguration),
    Text(&'static str),
    Readable(&'static str),
    Unknown,
}

impl FileTypeSelection {
    fn new(file_type_suggestion: FileTypeSuggestion) -> Self {
        match file_type_suggestion {
            FileTypeSuggestion::Directory => Self::Directory("Directory"),
            FileTypeSuggestion::Executable => {
                Self::Executable("Executable", ExecutableConfiguration::default())
            }
            FileTypeSuggestion::Text => Self::Text("Text"),
            FileTypeSuggestion::Readable => Self::Readable("Readable file (json, yaml, etc.)"),
            FileTypeSuggestion::Unknown => Self::Unknown,
        }
    }
    fn to_text(&self) -> &str {
        match self {
            FileTypeSelection::Directory(s) => s,
            FileTypeSelection::Executable(s, _) => s,
            FileTypeSelection::Text(s) => s,
            FileTypeSelection::Readable(s) => s,
            FileTypeSelection::Unknown => "...",
        }
    }

    pub fn show_interpreters(
        ui: &mut egui::Ui,
        cfg: &mut ExecutableConfiguration,
    ) -> egui::Response {
        let interpreter = &mut cfg.interpreter;
        let mut response = ui.selectable_value(
            interpreter,
            Some(ExecutableInterpreter("python3".to_string())),
            RichText::new("Python 3").size(32.),
        );

        response |= ui.selectable_value(
            interpreter,
            Some(ExecutableInterpreter("pwsh".to_string())),
            RichText::new("Powershell (pwsh)").size(32.),
        );

        response |= ui.selectable_value(
            interpreter,
            Some(ExecutableInterpreter("sh".to_string())),
            RichText::new("Shell").size(32.),
        );

        response |= ui.selectable_value(
            interpreter,
            Some(ExecutableInterpreter("bash".to_string())),
            RichText::new("Bash").size(32.),
        );

        // response |= ui.selectable_value(selected, FileTypeSelection::Unknown, "Unknown");

        response
    }
    /// Draws selectable buttons for each file type and updates `selected` on click.
    /// Returns a combined `Response` so you can check `.changed()`.
    pub fn show_selectables(ui: &mut egui::Ui, selected: &mut FileTypeSelection) -> egui::Response {
        let mut response = ui.selectable_value(
            selected,
            FileTypeSelection::Directory("Directory"),
            RichText::new("Directory").size(32.),
        );

        response |= ui.selectable_value(
            selected,
            FileTypeSelection::Executable("Executable", ExecutableConfiguration::default()),
            RichText::new("Executable").size(32.),
        );

        response |= ui.selectable_value(
            selected,
            FileTypeSelection::Text("Text"),
            RichText::new("Text").size(32.),
        );

        response |= ui.selectable_value(
            selected,
            FileTypeSelection::Readable("Readable"),
            RichText::new("Readable").size(32.),
        );

        // response |= ui.selectable_value(selected, FileTypeSelection::Unknown, "Unknown");

        response
    }
}

fn process_suggestion(suggestion: Res<FileTypeSuggestion>, mut commands: Commands) {
    commands.insert_resource(FileTypeSelection::new(suggestion.clone()));
}
fn check(ui: &mut egui::Ui, value: &mut bool) {
    ui.checkbox(value, "Checkbox");
}
// fn ui_header(ui: &mut egui::Ui, commands: &mut Commands) {
//     tui(ui, ui.id().with("selection_header"))
//         .reserve_available_space()
//         .style(Style {
//             flex_direction: FlexDirection::Row,
//             justify_content: Some(JustifyContent::FlexEnd),
//             align_items: Some(AlignItems::Center),
//             ..Default::default()
//         })
//         .show(|tui| {
//             tui.ui(|ui| {
//                 let back = egui::Button::new(egui::RichText::new("← Back").size(32.0).strong())
//                     .fill(egui::Color32::DARK_RED);
//
//                 if padded_button(ui, back, egui::Vec2::new(25.0, 12.0)).clicked() {
//                     commands.remove_resource::<FileTypeSuggestion>();
//                     commands.remove_resource::<DroppedFile>();
//                     commands.remove_resource::<FileTypeSelection>();
//                     commands.set_state(VisualizerState::Input);
//                 }
//             });
//         });
// }
// use egui_taffy::tui;
// use egui_taffy::taffy::prelude::*;
// fn ui_header(tui: &mut egui_taffy::Tui, commands: &mut Commands) {
//     // tui.add(|tui| {
//     //     tui.style(Style { ..default() });
//     // });
//     tui.ui(|ui| {
//         // TODO(taffy): header should be a flex row, right-aligned
//         // This likely needs a nested `tui(...)` or child container
//         // For now, keep it simple and egui-based
//
//         ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
//             let back = egui::Button::new(egui::RichText::new("← Back").size(32.0).strong())
//                 .fill(egui::Color32::DARK_RED);
//
//             if padded_button(ui, back, egui::Vec2::new(25.0, 12.0)).clicked() {
//                 commands.remove_resource::<FileTypeSuggestion>();
//                 commands.remove_resource::<DroppedFile>();
//                 commands.remove_resource::<FileTypeSelection>();
//                 commands.set_state(VisualizerState::Input);
//             }
//         });
//     });
// }
// fn central_frame() -> egui::Frame {
//     let color = Color32::from_rgb(10, 10, 30);
//     egui::Frame {
//         outer_margin: egui::Margin::symmetric(50, 50),
//         inner_margin: egui::Margin::symmetric(100, 50),
//         corner_radius: egui::CornerRadius::same(5),
//         // shadow: egui::Shadow::default(),
//         fill: egui::Color32::BLACK,
//         stroke: egui::Stroke::new(2.0, color),
//         ..Default::default()
//     }
// }
//
// fn ui_file_type_selector(tui: &mut egui_taffy::Tui, selection: &mut FileTypeSelection) {
//     tui.ui(|ui| {
//         ui.label(egui::RichText::new("File Type :").size(40.));
//
//         egui::ComboBox::from_id_salt("FILETYPE_SELECTOR")
//             .selected_text(egui::RichText::new(selection.to_text()).size(38.))
//             .show_ui(ui, |ui| {
//                 FileTypeSelection::show_selectables(ui, selection);
//             });
//     });
// }
//
// fn ui_footer(tui: &mut egui_taffy::Tui, dropped: &DroppedFile, selection: &FileTypeSelection) {
//     tui.ui(|ui| {
//         if let FileTypeSelection::Executable(_, cfg) = selection {
//             separator(ui);
//
//             let command = format!(
//                 "Executable command : {}{}",
//                 cfg.interpreter
//                     .as_ref()
//                     .map_or(String::new(), |itp| itp.to_display_string()),
//                 dropped.0.file_name().unwrap_or_default().display(),
//             );
//
//             ui.code(egui::RichText::new(command).size(22.));
//         }
//     });
// }
//
// fn ui_executable_block(tui: &mut egui_taffy::Tui, cfg: &mut ExecutableConfiguration) {
//     tui.ui(|ui| {
//         separator(ui);
//
//         ui.toggle_value(
//             &mut cfg.use_interpreter,
//             egui::RichText::new("Enable Interpreter").size(40.),
//         );
//
//         if !cfg.use_interpreter {
//             cfg.interpreter = None;
//             return;
//         }
//
//         ui.horizontal(|ui| {
//             ui.label(
//                 egui::RichText::new("Interpreter Type :")
//                     .size(32.)
//                     .underline(),
//             );
//
//             egui::ComboBox::from_id_salt("INTERPRETER_TYPE_SELECTOR")
//                 .selected_text(
//                     egui::RichText::new(
//                         cfg.interpreter
//                             .as_ref()
//                             .map_or("No interpreter selected...", |i| &i.0),
//                     )
//                     .size(32.),
//                 )
//                 .show_ui(ui, |ui| {
//                     FileTypeSelection::show_interpreters(ui, cfg);
//                 });
//         });
//
//         separator(ui);
//         check(ui, &mut cfg.check);
//     });
// }
//
// fn ui_selection_menu(
//     mut commands: Commands,
//     dropped: Res<DroppedFile>,
//     mut selection: ResMut<FileTypeSelection>,
//     mut ctx: EguiContexts,
// ) -> Result {
//     let ctx = ctx.ctx_mut()?;
//
//     egui::CentralPanel::default()
//         .frame(central_frame())
//         .show(ctx, |ui| {
//             tui(ui, ui.id().with("selection_root"))
//                 .reserve_available_space()
//                 .style(Style {
//                     flex_direction: FlexDirection::Column,
//                     align_self: Some(egui_taffy::taffy::AlignSelf::Center),
//
//                     // flex_grow: 1.,
//                     size: taffy::Size {
//                         width: taffy::prelude::percent(100.),
//                         height: taffy::prelude::percent(100.),
//                     },
//                     // TODO(taffy): vertical spacing between sections
//                     // Likely via gap or padding on this container
//                     ..Default::default()
//                 })
//                 .show(|tui| {
//                     // Header (Back button)
//                     ui_header(tui, &mut commands);
//
//                     // Separator
//                     tui.ui(separator);
//
//                     // File type selector
//                     ui_file_type_selector(tui, &mut selection);
//
//                     // Conditional block
//                     if let FileTypeSelection::Executable(_, cfg) = &mut *selection {
//                         // TODO(taffy): maybe visually group this block
//                         ui_executable_block(tui, cfg);
//                     }
//
//                     // TODO(taffy): want footer to stick to bottom
//                     // This likely requires a flex-grow spacer node
//
//                     // Footer
//                     ui_footer(tui, &dropped, &selection);
//                 });
//         });
//
//     Ok(())
// }

// fn ui_selection_menu(
//     mut commands: Commands,
//     dropped: Res<DroppedFile>,
//     mut selection: ResMut<FileTypeSelection>,
//     mut ctx: EguiContexts, //&mut egui::Context,
// ) -> Result {
//     let ctx = ctx.ctx_mut()?;
//     // Ensure resource exists first
//     let selected = &mut *selection;
//     let color = Color32::from_rgb(10, 10, 30);
//     let frame = egui::Frame {
//         outer_margin: egui::Margin::symmetric(50, 50),
//         inner_margin: egui::Margin::symmetric(100, 50),
//         corner_radius: egui::CornerRadius::same(5),
//         // shadow: egui::Shadow::default(),
//         fill: egui::Color32::BLACK,
//         stroke: egui::Stroke::new(2.0, color),
//         ..Default::default()
//     };
//
//     egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
//         ui.vertical_centered(|ui| {
//             ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
//                 let b = egui::Button::new(RichText::new("← Back").size(32.).strong())
//                     .fill(Color32::DARK_RED);
//                 if padded_button(ui, b, egui::Vec2::new(25., 12.)).clicked() {
//                     commands.remove_resource::<FileTypeSuggestion>();
//                     commands.remove_resource::<DroppedFile>();
//                     commands.remove_resource::<FileTypeSelection>();
//                     //TODO: Could be some previous steps; that could be handled thanks to message
//                     //readers
//                      commands.set_state(VisualizerState::Input);
//                 }
//             });
//             separator(ui);
//             // ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
//             // ui.horizontal(|ui| {
//             ui.label(RichText::new("File Type :").size(40.));
//             ComboBox::from_id_salt("FILETYPE_SELECTOR")
//                 .selected_text(RichText::new(selected.to_text()).size(38.))
//                 .show_ui(ui, |ui| {
//                     FileTypeSelection::show_selectables(ui, selected);
//                     // if r.changed() {
//                     //     commands.insert_resource(selected.clone());
//                     // }
//                     // });
//                 });
//             // let mut view = selected.clone();
//             //FIXME: Handle other cases
//             #[allow(clippy::single_match)]
//             match selected {
//                 // FileTypeSelection::Directory(_) => todo!(),
//                 FileTypeSelection::Executable(_name, executable_cfg) => {
//                     separator(ui);
//                     ui.with_layout(
//                         Layout::left_to_right(Align::Min).with_main_wrap(true),
//                         |ui| {
//                             ui.toggle_value(
//                                 &mut executable_cfg.use_interpreter,
//                                 RichText::new("Enable Interpreter").size(40.),
//                             );
//                         },
//                     );
//                     if !executable_cfg.use_interpreter {
//                         executable_cfg.interpreter = None;
//                     }
//                     if executable_cfg.use_interpreter {
//                         ui.with_layout(
//                             Layout::left_to_right(Align::Min), //.with_main_wrap(true),
//                             |ui| {
//                                 // ui.horizontal(|ui| {
//                                 ui.label(RichText::new("Interpreter Type :").size(32.).underline());
//                                 ComboBox::from_id_salt("INTERPRETER_TYPE_SELECTOR")
//                                     .selected_text(
//                                         RichText::new(
//                                             executable_cfg
//                                                 .interpreter
//                                                 .as_ref()
//                                                 .map_or("No interpreter selected...", |itp| &itp.0),
//                                         )
//                                         .size(32.),
//                                     )
//                                     .show_ui(ui, |ui| {
//                                         let r = FileTypeSelection::show_interpreters(
//                                             ui,
//                                             executable_cfg,
//                                         );
//                                         if r.changed() {
//                                             // commands.insert_resource(selected.clone());
//                                         }
//                                     });
//                             },
//                         );
//                     };
//
//                     separator(ui);
//                     check(ui, &mut executable_cfg.check);
//                     separator(ui);
//                     ui.with_layout(Layout::bottom_up(Align::Min).with_main_wrap(true), |ui| {
//                         ui.code(
//                             RichText::new(format!(
//                                 "Executable command : {}{}",
//                                 executable_cfg
//                                     .interpreter
//                                     .as_ref()
//                                     .map_or("".to_string(), |itp| itp.to_display_string()),
//                                 dropped.0.file_name().unwrap_or_default().display(),
//                             ))
//                             .size(22.),
//                         );
//                         separator(ui);
//                     });
//                 }
//                 // FileTypeSelection::Executable(_, _) => todo!(),
//                 FileTypeSelection::Text(_) => {
//                     ui.code(RichText::new("Reading text file at :").size(22.));
//                     ui.code(RichText::new(dropped.0.to_str().unwrap_or("")).size(22.));
//                 }
//                 // FileTypeSelection::Readable(_) => todo!(),
//                 // FileTypeSelection::Unknown => todo!(),
//                 _ => {}
//             }
//         });
//     });
//     Ok(())
// }
fn ui_dnd(hovered: Option<Res<HoveredFile>>, mut contexts: EguiContexts) -> Result {
    let (color, text) = match hovered {
        //TODO: Do away with this clone. There is NO WAY this is the right way to do it.
        Some(file) => (egui::Color32::GREEN, file.0.clone()),
        None => (
            egui::Color32::WHITE,
            "Drag and drop a file or executable here...".to_string(),
        ),
    };
    let frame = egui::Frame {
        outer_margin: egui::Margin::symmetric(50, 50),
        corner_radius: egui::CornerRadius::same(5),
        // shadow: egui::Shadow::default(),
        fill: egui::Color32::from_gray(30),
        stroke: egui::Stroke::new(2.0, color),
        ..Default::default()
    };
    egui::CentralPanel::default()
        .frame(frame)
        .show(contexts.ctx_mut()?, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(RichText::new(&text).size(32.));
            });
        });
    Ok(())
}
/// Simple UI for the Ui state
fn ui_system(
    // mut commands: Commands,
    mut contexts: EguiContexts,
    mut ui_size: ResMut<UiSize>,
    mut writer: MessageWriter<ViewportChanged>,
    // mut camera: Single<&mut Camera, Without<EguiContext>>,
    window: Single<&mut Window, With<PrimaryWindow>>,
    // mut next_state: ResMut<NextState<AppState>>
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let mut top = egui::TopBottomPanel::top("main_menu")
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("Storyteller").size(32.));
                ui.label(RichText::new("Every problem has a story to show...").size(14.));
            })
        })
        .response
        .rect
        .height();
    // if let Some(dropped) = dropped
    //     && let Some(suggestion) = suggestion
    // {
    //     ui_selection_menu(&mut commands, dropped, suggestion, selection, ctx);
    // }
    let mut bottom = egui::TopBottomPanel::bottom("bottom_menu")
        .resizable(true)
        .show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                if ui.button(RichText::new("Load")).clicked() {};
                ui.separator();
                if ui.button(RichText::new("Start")).clicked() {};
                ui.separator();
            });
        })
        .response
        .rect
        .height();
    top *= window.scale_factor();
    bottom *= window.scale_factor();
    if (top - ui_size.top).abs() > 0.01 || (bottom - ui_size.bottom).abs() > 0.01 {
        ui_size.top = top;
        ui_size.bottom = bottom;
        writer.write(ViewportChanged {
            id: ViewportId::Ui,
            top,
            bottom,
            left: 0.,
            right: 0.,
        });
    }
    //FIXME: Fix the window dying on minimize

    // let pos = UVecpub 2::new(0, top as u32);
    // let size = UVec2::new(window.physical_width(), window.physical_height())
    //     - pos
    //     - UVec2::new(0, bottom as u32);
    // camera.viewport = Some(Viewport {
    //     physical_position: pos,
    //     physical_size: size,
    //     ..default()
    // });
    Ok(())
}

/// Space toggles between Grid and Ui
fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<UiStatus>>,
    mut next_state: ResMut<NextState<UiStatus>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let next = match state.get() {
            UiStatus::Visible => UiStatus::Invisible,
            UiStatus::Invisible => UiStatus::Visible,
        };
        info!("Switching to {:?}", next);
        next_state.set(next);
    }
}
