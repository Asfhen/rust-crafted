use std::f32::consts::FRAC_PI_2;
use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode};

pub const DEFAULT_CAMERA_SENS: f32 = 0.005;

#[derive(Component, Default)]
pub struct PlayerController {
    pub yaw: f32,
    pub pitch: f32,
    pub cursor_locked: bool,
}

pub fn handle_player_mouse_move(
    mut query: Query<(&mut PlayerController, &mut Transform)>,
    mut mouse_motion_event_reader: EventReader<MouseMotion>,
    mut window: Query<&mut Window>,
) {
    let (mut controller, mut transform) = query.single_mut().unwrap();
    let mut delta = Vec2::ZERO;

    if controller.cursor_locked {
        for mouse_move in mouse_motion_event_reader.read() {
            delta += mouse_move.delta;
        }
    }

    let mut first_win = window.single_mut().unwrap();
    first_win.cursor_options.visible = !controller.cursor_locked;
    first_win.cursor_options.grab_mode = if controller.cursor_locked {
        CursorGrabMode::Locked
    } else {
        CursorGrabMode::None
    };

    if delta == Vec2::ZERO {
        return;
    }

    let mut new_pitch = delta.y.mul_add(DEFAULT_CAMERA_SENS, controller.pitch);
    let new_yaw = delta.x.mul_add(-DEFAULT_CAMERA_SENS, controller.yaw);

    new_pitch = new_pitch.clamp(-FRAC_PI_2, FRAC_PI_2);

    controller.yaw = new_yaw;
    controller.pitch = new_pitch;

    transform.rotation = 
        Quat::from_axis_angle(Vec3::Y, new_yaw) * Quat::from_axis_angle(-Vec3::X, new_pitch);
}

pub fn handle_player_input(
    mut query: Query<(&mut PlayerController, &mut Transform)>,
    keys: Res<ButtonInput<KeyCode>>,
    btns: Res<ButtonInput<MouseButton>>,
) {
    let (mut controller, mut transform) = query.single_mut().unwrap();

    if btns.just_pressed(MouseButton::Left) {
        controller.cursor_locked = true;
    }

    if keys.just_pressed(KeyCode::Escape) {
        controller.cursor_locked = false;
    }

    let mut direction = Vec3::ZERO;

    let forward = transform.rotation.mul_vec3(Vec3::Z).normalize() * Vec3::new(1.0, 0., 1.0);
    let right = transform.rotation.mul_vec3(Vec3::X).normalize();

    let mut acceleration = 1.0f32;

    if keys.pressed(KeyCode::KeyW) {
        direction.z -= 1.0;
    }

    if keys.pressed(KeyCode::KeyS) {
        direction.z += 1.0;
    }

    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }

    if keys.pressed(KeyCode::Space) {
        direction.y += 1.0;
    }

    if keys.pressed(KeyCode::ShiftLeft) {
        direction.y -= 1.0;
    }

    if keys.pressed(KeyCode::ControlLeft) {
        acceleration *= 8.0;
    }

    if direction == Vec3::ZERO {
        return;
    }

    // hardcoding 0.10 as a factor for now to not go zoomin across the world.
    transform.translation += direction.x * right * acceleration
        + direction.z * forward * acceleration
        + direction.y * Vec3::Y * acceleration;
}
