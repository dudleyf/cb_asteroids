use bevy::{prelude::*, window::PrimaryWindow};

use crate::ui::pause::Pausable;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (linear_movement, spin, wrapping_movement).run_if(resource_equals(Pausable::NotPaused)),
        );
    }
}

#[derive(Component)]
pub struct LinearMovement {
    pub movement_factor: Vec2,
    pub movement_direction: Quat,
}

fn linear_movement(mut objects: Query<(&mut Transform, &LinearMovement)>, time: Res<Time>) {
    for (
        mut transform,
        LinearMovement {
            movement_factor: preexisting_movement_factor,
            movement_direction,
        },
    ) in &mut objects
    {
        let object_facing_direction = *movement_direction * Vec3::Y;
        let translation_delta =
            *preexisting_movement_factor + object_facing_direction.xy() * time.delta_seconds();
        transform.translation.x += translation_delta.x;
        transform.translation.y += translation_delta.y;
    }
}

#[derive(Component)]
pub struct Spin(pub f32);

fn spin(mut spinnable: Query<(&Spin, &mut Transform)>, time: Res<Time>) {
    for (spin, mut transform) in &mut spinnable {
        transform.rotate_z(spin.0 * time.delta_seconds());
    }
}

#[derive(Component)]
pub struct WrappingMovement;

fn wrapping_movement(
    mut wrappers: Query<&mut Transform, With<WrappingMovement>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = window.get_single() else {
        warn!("No primary window, not implementing wrapping movement");
        return;
    };
    let width = window.resolution.width() / 2.0;
    let height = window.resolution.height() / 2.0;
    for mut transform in &mut wrappers {
        let x = transform.translation.x;
        let y = transform.translation.y;
        if x > width {
            transform.translation.x -= window.resolution.width();
        } else if x < -width {
            transform.translation.x += window.resolution.width();
        }
        if y > height {
            transform.translation.y -= window.resolution.height();
        } else if y < -height {
            transform.translation.y += window.resolution.height();
        }
    }
}
