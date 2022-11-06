use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use std::f32::consts::PI;

use gamedebug::GameDebugPlugin;
use crate::skybox::SkyboxPlugin;

mod orbitcamera;
mod gamedebug;
mod skybox;

fn main() {
    App::new()
        //add config resources
        .insert_resource(Msaa {samples: 4})
        .insert_resource(WindowDescriptor{
            title: "bevy lightbike".to_string(),
            width: 920.0,
            height: 640.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::WHITE))
        //.insert_resource(Score::default())
        //bevy itself
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(GameDebugPlugin)
        .add_plugin(SkyboxPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(setup)
        .run();
}

fn setup_camera(
    mut commands: Commands
) {
    commands.
        spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(9.0, 2.0, -9.0).looking_at(Vec3::new(0.,0.,0.), Vec3::Y),
            ..Default::default()
        })
        .insert(UiCameraConfig {
            show_ui: true,
            ..default()
        })
        .insert(Name::new("MainCamera"));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    //light
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 11.6, -15.1),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });

    //platform
    let texture_handle = asset_server.load("images/grid.png");

    let platform_length = Vec3::new(200.0,0.1,200.0);
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(
                                                platform_length.x,
                                                platform_length.y,
                                                platform_length.z))),
            material: materials.add( StandardMaterial{
                //base_color: Color::rgb(0.0, 0.0, 0.2),
                base_color_texture: Some(texture_handle.clone()),
                double_sided: true,
                ..Default::default()
            }),
            transform: Transform {
                translation: Vec3::new(0.0, -platform_length.y, 0.0),
                rotation: Quat::from_rotation_x(0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Sleeping::disabled())
        .insert(Collider::cuboid(
            platform_length.x/2.0,
            platform_length.y/2.0,
            platform_length.z/2.0
        ));

    //walls
    let wall_height = 20.0;
    let wall_position = vec![
                                Vec3::new(0.0,wall_height/2.0,platform_length.z/2.0),
                                Vec3::new(0.0,wall_height/2.0,-platform_length.z/2.0),
                                Vec3::new(platform_length.x/2.0,wall_height/2.0,0.0),
                                Vec3::new(-platform_length.x/2.0,wall_height/2.0,0.0),
                        ];
    let wall_size = vec![
                                Vec3::new(platform_length.x,wall_height,platform_length.y),
                                Vec3::new(platform_length.x,wall_height,platform_length.y),
                                Vec3::new(platform_length.y,wall_height,platform_length.z),
                                Vec3::new(platform_length.y,wall_height,platform_length.z)
                        ];
    let texture_handle = asset_server.load("images/wall1.png");
    for i in 0..wall_size.len(){
        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box::new(
                    wall_size[i].x,
                    wall_size[i].y,
                    wall_size[i].z))),
                material: materials.add( StandardMaterial{
                    //base_color: Color::rgb(0.0, 0.0, 0.2),
                    base_color_texture: Some(texture_handle.clone()),
                    double_sided: true,
                    ..Default::default()
                }),
                transform: Transform {
                    translation: wall_position[i].clone(),
                    rotation: Quat::from_rotation_x(0.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(RigidBody::Fixed)
            .insert(Sleeping::disabled())
            .insert(Collider::cuboid(
                platform_length.x/2.0,
                platform_length.y/2.0,
                platform_length.z/2.0
            ));
    }

    //bike
    commands.spawn_bundle(SceneBundle {
        scene: asset_server.load("models/bike.glb#Scene0"),
        transform:Transform::from_translation(Vec3::ZERO),
        ..Default::default()
    });
}

