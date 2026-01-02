use bevy::{camera::Viewport, prelude::*, window::PrimaryWindow};
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ViewportId {
    Ui,
    Primary,
    Secondary(u8),
}

use std::collections::HashMap;

#[derive(Resource, Debug, Default)]
pub struct Viewports {
    viewports: HashMap<ViewportId, Viewport>,
}
impl Viewports {
    pub fn get_mut(&mut self) -> MutableViewports<'_> {
        MutableViewports(self)
    }
}
/// Temporary façade that exposes mutation. Should only be consumed in one place - problematic
/// side effects are too easy to implement.
pub struct MutableViewports<'a>(&'a mut Viewports);

impl<'a> std::ops::Deref for MutableViewports<'a> {
    type Target = HashMap<ViewportId, Viewport>;

    fn deref(&self) -> &Self::Target {
        &self.0.viewports
    }
}

impl<'a> std::ops::DerefMut for MutableViewports<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0.viewports
    }
}

#[derive(Message)]
pub struct ViewportChanged {
    pub id: ViewportId, // Primary, Secondary, Debug, etc.
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}
fn viewport_event_system(
    mut events: MessageReader<ViewportChanged>,
    mut viewports: ResMut<Viewports>,
    //TODO: Handle multiple windows ?
    window: Single<&mut Window, With<PrimaryWindow>>,
    // windows: Res<Windows>,
) {
    // let window = windows.get_primary().unwrap();
    let win_width = window.physical_width() as f32;
    let win_height = window.physical_height() as f32;

    for ev in events.read() {
        let left = ev.left.clamp(0.0, win_width);
        let right = ev.right.clamp(0.0, win_width);
        let top = ev.top.clamp(0.0, win_height);
        let bottom = ev.bottom.clamp(0.0, win_height);

        let pos = UVec2::new(left as u32, top as u32);
        let size = UVec2::new(
            (win_width - left - right).max(0.0) as u32,
            (win_height - top - bottom).max(0.0) as u32,
        );

        viewports.get_mut().insert(
            ev.id,
            Viewport {
                physical_position: pos,
                physical_size: size,
                ..Default::default()
            },
        );
    }
}
#[derive(Resource, Debug, Default)]
pub struct UiSize {
    pub top: f32,
    pub bottom: f32,
}
