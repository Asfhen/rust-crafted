use bevy::{log::LogPlugin, prelude::*, render::view::RenderLayers};
use camera::CameraPlugin;
use voxel_engine::common::{
    error::{log_error, ErrorEvent},
    logging::setup_file_logging,
};

mod camera;

fn main() {
    let _guard = setup_file_logging();
    App::new()
        .add_event::<ErrorEvent>()
        .add_systems(Update, log_error)
        .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
        .add_plugins(CameraPlugin)
        .add_systems(Startup, setup)
        .register_type::<Option<Handle<Image>>>()
        .register_type::<AlphaMode>()
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut error_events: EventWriter<ErrorEvent>,
) {
    let box_size = 2.0;
    let box_thickness = 0.15;
    let box_offset = (box_size + box_thickness) / 2.0;

    // Test error handling
    error_events.write(ErrorEvent(voxel_engine::common::error::GameError::Client(
        voxel_engine::common::error::ClientError::AssetError("Teste error!!!!".to_string()),
    )));

    // left - red
    let mut transform = Transform::from_xyz(-box_offset, box_offset, 0.0);
    transform.rotate(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(box_size, box_thickness, box_size))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.63, 0.065, 0.05),
            ..Default::default()
        })),
        transform,
    ));
    // right - green
    let mut transform = Transform::from_xyz(box_offset, box_offset, 0.0);
    transform.rotate(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(box_size, box_thickness, box_size))),
        transform,
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.14, 0.45, 0.091),
            ..Default::default()
        })),
        RenderLayers::layer(0),
    ));
    // bottom - white
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(
            box_size + 2.0 * box_thickness,
            box_thickness,
            box_size,
        ))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.725, 0.71, 0.68),
            ..Default::default()
        })),
        RenderLayers::layer(0),
    ));
    // top - white
    let transform = Transform::from_xyz(0.0, 2.0 * box_offset, 0.0);
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(
            box_size + 2.0 * box_thickness,
            box_thickness,
            box_size,
        ))),
        transform,
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.725, 0.71, 0.68),
            ..Default::default()
        })),
        RenderLayers::layer(0),
    ));
    // back - white
    let mut transform = Transform::from_xyz(0.0, box_offset, -box_offset);
    transform.rotate(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(
            box_size + 2.0 * box_thickness,
            box_thickness,
            box_size + 2.0 * box_thickness,
        ))),
        transform,
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.725, 0.71, 0.68),
            ..Default::default()
        })),
        RenderLayers::layer(0),
    ));

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.02,
        affects_lightmapped_meshes: true,
    });
    // top light
    commands
        .spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(0.4, 0.4))),
            Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::ONE,
                Quat::from_rotation_x(std::f32::consts::PI),
                Vec3::new(0.0, box_size + 0.5 * box_thickness, 0.0),
            )),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::WHITE,
                emissive: LinearRgba::WHITE * 100.0,
                ..Default::default()
            })),
            RenderLayers::layer(0),
        ))
        .with_children(|builder| {
            builder.spawn((
                PointLight {
                    color: Color::WHITE,
                    intensity: 25000.0,
                    ..Default::default()
                },
                Transform::from_translation((box_thickness + 0.05) * Vec3::Y),
                RenderLayers::layer(0),
            ));
        });
    // directional light
    commands.spawn((
        DirectionalLight {
            illuminance: 2000.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::PI / 2.0)),
    ));
}
