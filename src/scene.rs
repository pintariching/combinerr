use bevy::prelude::*;

pub fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 20.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Name::new("Ground"));

    commands
        .spawn_bundle(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 40000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -2.3, 0.0, 0.0)),
            ..default()
        })
        .insert(Name::new("Sun"));
}
