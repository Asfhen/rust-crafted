use bevy::{input::{keyboard::KeyboardInput, mouse::MouseMotion, ButtonState}, prelude::*};
use bevy_rapier3d::prelude::*;
use voxel_engine::common::components::player::Player;

const PLAYER_HEIGHT: f32 = 1.8;
const PLAYER_RADIUS: f32 = 0.3;

#[derive(Component)]
pub struct PlayerCamera;

pub fn spawn_player(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
        Player::default(),
        RigidBody::Dynamic,
        Collider::capsule_y(PLAYER_HEIGHT * 0.5, PLAYER_RADIUS),
        Velocity::zero(),
        LockedAxes::ROTATION_LOCKED,
        GravityScale(2.0),
        Transform::from_xyz(0.0, 100.0, 0.0),
        Name::new("Player")
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, PLAYER_HEIGHT * 0.9, 0.0),
        PlayerCamera,
        Name::new("PlayerCamera")
    ));
}

pub fn player_movement(
    _time: Res<Time>,
    mut keyboard_input: EventReader<KeyboardInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut player_query: Query<(&mut Velocity, &mut Transform, &Player)>,
    mut camera_query: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
) {
    let (mut velocity, mut player_transform, player) = player_query.single_mut().unwrap();
    let mut camera_transform = camera_query.single_mut().unwrap();

    for event in mouse_motion_events.read() {
        let rotation_y = -event.delta.x * 0.001;
        player_transform.rotate_y(rotation_y);
        camera_transform.rotate_y(rotation_y);
    }

    let rotation_x = mouse_motion_events.read()
        .map(|e| -e.delta.x * 0.001)
        .sum::<f32>();
    camera_transform.rotate_local_x(rotation_x.clamp(-0.5, 0.5));

    let mut direction = Vec3::ZERO;
    for key in keyboard_input.read() {
        if key.state == ButtonState::Released {
            continue;
        }

        match key.key_code {
            KeyCode::KeyW => direction += *player_transform.forward(),
            KeyCode::KeyS => direction += *player_transform.back(),
            KeyCode::KeyA => direction += *player_transform.left(),
            KeyCode::KeyD => direction += *player_transform.right(),
            KeyCode::Space => if player.is_on_ground { velocity.linvel.y += player.jump_strength; }
            _ => continue,
        }

        if direction.length_squared() > 0.0 {
            direction.y = 0.0;
            direction = direction.normalize() * player.movement_speed;
            velocity.linvel = Vec3::new(direction.x, velocity.linvel.y, direction.z)
        }
    }
}

pub fn update_ground_state(
    mut player_query: Query<(&mut Player, &Velocity)>,
    rapier_context: ReadRapierContext,
) {
    for (mut player, _velocity) in player_query.iter_mut() {
        let ray_origin = Vec3::new(0.0, -PLAYER_HEIGHT * 0.45, 0.0);
        let ray_direction = Vec3::NEG_Y;
        let max_distance = 0.1;

        let hit = rapier_context
            .single()
            .unwrap()
            .cast_ray(
                ray_origin,
                ray_direction,
                max_distance,
                true,
                QueryFilter::only_dynamic(),
            );
        
        let mut is_on_ground = false;
        
        for angle in [0.0, 90.0, 180.0, 270.0] as [f32; 4] {
            let rad = angle.to_radians();
            let direction = Vec3::new(rad.sin(), -0.7, rad.cos()); // -0.7 para inclinação
            
            if let Some((_, toi)) = rapier_context.single().unwrap().cast_ray(
                ray_origin,
                direction,
                0.5, // Maior distância para rampas
                true,
                QueryFilter::default()
            ) {
                if toi < 0.5 {
                    is_on_ground = true;
                    break;
                }
            }
        }
        
        player.is_on_ground = hit.is_some() || is_on_ground;
    }
}

pub fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>
) {
    if let Ok(player_transform) = player_query.single() {
        let mut camera_transform = camera_query.single_mut().unwrap();

        camera_transform.translation = player_transform.translation + Vec3::Y * PLAYER_HEIGHT * 0.9;
    }
}
