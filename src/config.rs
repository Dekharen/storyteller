use bevy::prelude::*;

#[derive(Resource)]
pub struct AnimationConfig {
    /// Seconds per tick for your simulation (e.g. 0.5 = 2 ticks per second)
    pub tick_rate: f32,
    /// Global animation speed multiplier
    pub animation_speed: f32,
    /// The interpolation type
    pub interpolation: InterpolationKind,
}

#[derive(Clone, Copy, Debug)]
pub enum InterpolationKind {
    Linear,
    Exponential { exponent: f32 },
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            tick_rate: 0.5,
            animation_speed: 1.0,
            interpolation: InterpolationKind::Exponential { exponent: 2.0 },
        }
    }
}
