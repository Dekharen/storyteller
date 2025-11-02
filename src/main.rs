use std::any::TypeId;

use bevy::camera::Viewport;
use bevy::camera::visibility::RenderLayers;
use bevy::input::mouse::{MouseMotion, MouseWheel};
// use bevy::hierarchy::DespawnRecursiveExt;
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::egui::RichText;
// use bevy::render::camera::Camera3d;
// use bevy::render::view::{ComputedVisibility, Visibility};
use bevy_egui::{
    EguiContext, EguiContexts, EguiGlobalSettings, EguiPlugin, EguiPrimaryContextPass,
    PrimaryEguiContext, egui,
};
use storyteller::visualization::{
    LoadVisualization, TaggedEntity, VisualizerState, file_drop, load_visualization_system,
    unload_visualization_system,
};
#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
enum UiStatus {
    #[default]
    Visible,
    Invisible,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum VisualizationSystemSet {
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
        .add_plugins(EguiPlugin::default())
        .init_state::<VisualizerState>()
        .init_state::<UiStatus>()
        .insert_resource(TickTimer(Timer::from_seconds(0.01, TimerMode::Repeating)))
        .add_message::<LoadVisualization>()
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
        .add_systems(OnEnter(UiStatus::Invisible), cleanup_ui)
        // --- UI ---
        .add_systems(
            EguiPrimaryContextPass,
            ui_system.run_if(in_state(UiStatus::Visible)),
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
    // commands.spawn((
    //     // The `PrimaryEguiContext` component requires everything needed to render a primary context.
    //     PrimaryEguiContext,
    //     Camera2d,
    //     // Setting RenderLayers to none makes sure we won't render anything apart from the UI.
    //     RenderLayers::none(),
    //     Camera {
    //         order: 1,
    //         output_mode: bevy::camera::CameraOutputMode::Write {
    //             blend_state: Some(bevy::render::render_resource::BlendState::ALPHA_BLENDING),
    //             clear_color: ClearColorConfig::None,
    //         },
    //         clear_color: ClearColorConfig::Custom(Color::NONE),
    //         ..default()
    //     },
    // ));
}

// fn animate_heights(
//     time: Res<Time>,
//     mut query: Query<(&mut Transform, &AnimatedHeight), With<Grounded>>,
// ) {
//     let dt = time.delta_secs();
//
//     for (mut transform, anim) in &mut query {
//         let current = transform.scale.y;
//         let diff = anim.target - current;
//         let step = anim.speed * dt;
//
//         // Move toward target, without overshooting
//         let new_y = if diff.abs() <= step {
//             anim.target
//         } else {
//             current + step * diff.signum()
//         };
//
//         transform.scale.y = new_y;
//         transform.translation.y = new_y / 2.0; // anchor to ground
//     }
// }

// fn setup_ui(mut commands: Commands) {
//     commands.spawn((Camera2d, VisEntity));
//     info!("entered Ui State");
// }

/// Setup 3D scene
fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // // Camera
    // commands.spawn((
    //     Camera3d::default(),
    //     Transform::from_xyz(5.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     GlobalTransform::default(),
    //     Visibility::default(),
    //     // ComputedVisibility::default(),
    //     VisEntity,
    // ));
    //
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

/// /// Clean up grid entities when leaving
/// fn cleanup_grid(mut commands: Commands, query: Query<Entity, With<TaggedEntity>>) {
///     for entity in &query {
///         commands.entity(entity).despawn();
///     }
///     info!("Exited Grid state");
/// }

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
    time: Res<Time>,
    // mut mouse_motion_events: MessageReader<MouseMotion>,
    // mut scroll_events: MessageReader<MouseWheel>,
    // mouse_buttons: Res<ButtonInput<MouseButton>>,
    // mut query: Query<(&mut Transform, &mut OrbitCamera)>,
    //
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
    // material_handles: Query<&MeshMaterial3d<StandardMaterial>>,
    //     time: Res<Time>,
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

            // }
        }

        // info!("Tick!");
    }
}

/// Simple UI for the Ui state
fn ui_system(
    mut contexts: EguiContexts,

    mut camera: Single<&mut Camera, Without<EguiContext>>,
    window: Single<&mut Window, With<PrimaryWindow>>,
    // mut next_state: ResMut<NextState<AppState>>
) -> Result {
    let mut top = egui::TopBottomPanel::top("main_menu")
        .show(contexts.ctx_mut()?, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("Storyteller").size(32.));
                ui.label(RichText::new("Every problem has a story to show...").size(14.));
            })
        })
        .response
        .rect
        .height();

    let mut bottom = egui::TopBottomPanel::bottom("bottom_menu")
        .resizable(true)
        .show(contexts.ctx_mut()?, |ui| {
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
    let pos = UVec2::new(0, top as u32);
    let size = UVec2::new(window.physical_width(), window.physical_height())
        - pos
        - UVec2::new(0, bottom as u32);
    camera.viewport = Some(Viewport {
        physical_position: pos,
        physical_size: size,
        ..default()
    });
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
//
//
// #[derive(Resource)]
// struct StoryEngine {
//     storyframe: Storyframe, // your library type
// }
// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_plugins(EguiPlugin::default())
//         .add_plugins(Wireframe2dPlugin::default())
//         // .add_systems(Startup, setup_camera_system)
//         .add_systems(Startup, setup)
//         .add_systems(
//             EguiPrimaryContextPass,
//             (ui_example_system, ui_bottom_system),
//         )
//         .add_systems(Update, animate_materials)
//         // .add_systems(Update, toggle_wireframe)
//         .run();
// }
//
// // fn setup_camera_system(mut commands: Commands) {
// //     commands.spawn(Camera2d);
// // }
// //
// fn ui_example_system(mut contexts: EguiContexts) -> Result {
//     egui::TopBottomPanel::top("main_menu").show(contexts.ctx_mut()?, |ui| {
//         ui.vertical_centered(|ui| {
//             ui.label(RichText::new("Storyteller").size(32.));
//             ui.label(RichText::new("Every problem has a story to show...").size(14.));
//         });
//     });
//
//     Ok(())
// }
//
// fn ui_bottom_system(mut contexts: EguiContexts) -> Result {
//     egui::TopBottomPanel::bottom("bottom_menu").show(contexts.ctx_mut()?, |ui| {
//         ui.horizontal_centered(|ui| {
//             if ui.button(RichText::new("Load")).clicked() {};
//             ui.separator();
//             if ui.button(RichText::new("Start")).clicked() {};
//             ui.separator();
//         });
//     });
//     Ok(())
// }
// fn setup(
//     mut commands: Commands,
//     // asset_server: Res<AssetServer>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     commands.spawn((
//         Camera3d::default(),
//         Transform::from_xyz(3.0, 1.0, 3.0).looking_at(Vec3::new(0.0, -0.5, 0.0), Vec3::Y),
//         // EnvironmentMapLight {
//         //     diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
//         //     specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
//         //     intensity: 2_000.0,
//         //     ..default()
//         // },
//     ));
//
//     let cube = meshes.add(Cuboid::new(0.5, 0.5, 0.5));
//
//     const GOLDEN_ANGLE: f32 = 137.507_77;
//
//     let mut hsla = Hsla::hsl(0.0, 1.0, 0.5);
//     for x in -1..2 {
//         for z in -1..2 {
//             commands.spawn((
//                 Mesh3d(cube.clone()),
//                 MeshMaterial3d(materials.add(Color::from(hsla))),
//                 Transform::from_translation(Vec3::new(x as f32, 0.0, z as f32)),
//             ));
//             hsla = hsla.rotate_hue(GOLDEN_ANGLE);
//         }
//     }
// }
//
// fn animate_materials(
//     material_handles: Query<&MeshMaterial3d<StandardMaterial>>,
//     time: Res<Time>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     for material_handle in material_handles.iter() {
//         if let Some(material) = materials.get_mut(material_handle)
//         // && let Color::Hsla(ref mut hsla) = material.base_color
//         {
//             material
//             // *hsla = hsla.rotate_hue(time.delta_secs() * 100.0);
//         }
//     }
// }
// // const X_EXTENT: f32 = 1200.;
// //
// // fn setup(
// //     mut commands: Commands,
// //     mut meshes: ResMut<Assets<Mesh>>,
// //     mut materials: ResMut<Assets<ColorMaterial>>,
// // ) {
// //
// //     let shapes = [
// //         meshes.add(Circle::new(50.0)),
// //         meshes.add(CircularSector::new(50.0, 1.0)),
// //         meshes.add(CircularSegment::new(50.0, 1.25)),
// //         meshes.add(Ellipse::new(25.0, 50.0)),
// //         meshes.add(Annulus::new(25.0, 50.0)),
// //         meshes.add(Capsule2d::new(25.0, 50.0)),
// //         meshes.add(Rhombus::new(75.0, 100.0)),
// //         meshes.add(Rectangle::new(50.0, 100.0)),
// //         meshes.add(RegularPolygon::new(50.0, 6)),
// //         meshes.add(Triangle2d::new(
// //             Vec2::Y * 50.0,
// //             Vec2::new(-50.0, -50.0),
// //             Vec2::new(50.0, -50.0),
// //         )),
// //         meshes.add(Segment2d::new(
// //             Vec2::new(-50.0, 50.0),
// //             Vec2::new(50.0, -50.0),
// //         )),
// //         meshes.add(Polyline2d::new(vec![
// //             Vec2::new(-50.0, 50.0),
// //             Vec2::new(0.0, -50.0),
// //             Vec2::new(50.0, 50.0),
// //         ])),
// //     ];
// //     let num_shapes = shapes.len();
// //
// //     for (i, shape) in shapes.into_iter().enumerate() {
// //         // Distribute colors evenly across the rainbow.
// //         let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);
// //
// //         commands.spawn((
// //
// //             Mesh2d(shape),
// //             MeshMaterial2d(materials.add(color)),
// //             Transform::from_xyz(
// //                 // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
// //                 -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
// //                 0.0,
// //                 0.0,
// //             ),
// //         ));
// //     }
// //
// //     commands.spawn((
// //         Text::new("Press space to toggle wireframes"),
// //         Node {
// //             position_type: PositionType::Absolute,
// //             top: px(12),
// //             left: px(12),
// //             ..default()
// //         },
// //     ));
// // }
// //
// // fn toggle_wireframe(
// //     mut wireframe_config: ResMut<Wireframe2dConfig>,
// //     keyboard: Res<ButtonInput<KeyCode>>,
// // ) {
// //     if keyboard.just_pressed(KeyCode::Space) {
// //         wireframe_config.global = !wireframe_config.global;
// //     }
// // }
