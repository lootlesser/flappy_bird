use bevy::prelude::*;

use crate::physics::velocity::Velocity;
pub struct GravityPlugin;

const FORCE_GRAVITY: f32 = 10.;

impl Plugin for GravityPlugin {

    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_gravity);
    }

}

#[derive(Component)]
pub struct Gravity {
    pub terminal_velocity: f32,
    pub multiplier: f32,
}

fn apply_gravity
(
    mut query: Query<(&Gravity, &mut Velocity)>,
)
{

    for (gravity, mut velocity) in query.iter_mut() {

        velocity.y = (velocity.y + FORCE_GRAVITY * gravity.multiplier).max(gravity.terminal_velocity)

    }

}
