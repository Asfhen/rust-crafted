use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    render::view::RenderLayers,
    window::{CursorGrabMode, PrimaryWindow},
};

#[derive(Component)]
pub struct MainCamera;

pub struct CameraPlugin;

#[derive(Reflect, Resource)]
#[reflect(Resource)]
pub struct MovementSettings {
    pub speed: f32,
    pub sensitivity: f32,
    pub vertical_multiplier: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            speed: 12.0,
            sensitivity: 0.00012,
            vertical_multiplier: 1.0,
        }
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MovementSettings>()
            .register_type::<MovementSettings>()
            .add_systems(Startup, setup_camera)
            .add_systems(Update, camera_movement)
            .add_systems(Update, handle_mouse_movement)
            .add_systems(Update, update_toggle_grab_cursor);
    }
}

fn setup_camera(
    mut commands: Commands,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        MainCamera,
        RenderLayers::layer(0),
    ));

    if let Ok(mut window) = primary_window.single_mut() {
        toggle_grab_cursor(&mut window);
    } else {
        warn!("Primary window not found for `initial_grab_cursor`!");
    }
}

fn toggle_grab_cursor(window: &mut Window) {
    match window.cursor_options.grab_mode {
        CursorGrabMode::None => {
            window.cursor_options.grab_mode = CursorGrabMode::Confined;
            window.cursor_options.visible = true;
        }
        _ => {
            window.cursor_options.grab_mode = CursorGrabMode::None;
            window.cursor_options.visible = true;
        }
    }
}

fn update_toggle_grab_cursor(
    keys: Res<ButtonInput<KeyCode>>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = primary_window.single_mut() {
        if keys.just_pressed(KeyCode::Escape) {
            toggle_grab_cursor(&mut window);
        }
    } else {
        warn!("Primary window not found for `update_toggle_grab_cursor`!");
    }
}

fn handle_mouse_movement(
    mut query: Query<&mut Transform, With<Camera>>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    settings: Res<MovementSettings>,
) {
    let Ok(window) = primary_window.single() else {
        return;
    };
    for mut transform in query.iter_mut() {
        let (_, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
        if mouse_motion.delta.length_squared() < 0.1 {
            return;
        }
        let (mut yaw, _, _) = transform.rotation.to_euler(EulerRot::YXZ);
        match window.cursor_options.grab_mode {
            CursorGrabMode::None => (),
            _ => {
                // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                let window_scale = window.height().min(window.width());
                pitch -= (settings.sensitivity * mouse_motion.delta.y * window_scale).to_radians();
                yaw -= (settings.sensitivity * mouse_motion.delta.x * window_scale).to_radians();
            }
        }
        pitch = pitch.clamp(-1.57, 1.57);

        // Order is important to prevent unintended roll
        transform.rotation =
            Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
    }
}

fn camera_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
    settings: Res<MovementSettings>,
) {
    if let Ok(window) = primary_window.single() {
        for mut transform in query.iter_mut() {
            let mut velocity = Vec3::ZERO;
            let local_z = transform.local_z();
            let forward = -Vec3::new(local_z.x, 0., local_z.z);
            let right = Vec3::new(local_z.z, 0., -local_z.x);

            for key in keys.get_pressed() {
                match window.cursor_options.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        let key = *key;
                        if key == KeyCode::KeyW {
                            velocity += forward;
                        } else if key == KeyCode::KeyS {
                            velocity -= forward;
                        } else if key == KeyCode::KeyA {
                            velocity -= right;
                        } else if key == KeyCode::KeyD {
                            velocity += right;
                        } else if key == KeyCode::Space {
                            velocity += Vec3::Y * settings.vertical_multiplier;
                        } else if key == KeyCode::ShiftLeft {
                            velocity -= Vec3::Y * settings.vertical_multiplier;
                        }
                    }
                }
            }

            transform.translation =
                transform.translation + velocity * settings.speed * time.delta_secs();
        }
    } else {
        warn!("Primary window not found for `player_move`!");
    }
}
