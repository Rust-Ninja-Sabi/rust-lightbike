use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use std::f32::consts::PI;
use bevy::render::render_resource::{AddressMode, SamplerDescriptor};
use bevy::render::texture::ImageSettings;

use gamedebug::GameDebugPlugin;
use crate::skybox::SkyboxPlugin;

mod orbitcamera;
mod gamedebug;
mod skybox;

enum Direction{
    forward,
    right,
    left,
    back
}

#[derive(Component)]
struct Camera{}

#[derive(Component)]
struct Bike{
    direction:Direction
}

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
        .insert_resource(ImageSettings {
            default_sampler: SamplerDescriptor {
                address_mode_u: AddressMode::Repeat,
                address_mode_v: AddressMode::Repeat,
                address_mode_w: AddressMode::Repeat,
                ..Default::default()
            },
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
        .add_system(move_bike)
        .add_system(move_camera)
        .run();
}

fn setup_camera(
    mut commands: Commands
) {
    commands.
        spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(1.11, 16.18, -36.57).looking_at(Vec3::new(0.,0.,0.), Vec3::Y),
            ..Default::default()
        })
        .insert(UiCameraConfig {
            show_ui: true,
            ..default()
        })
        .insert(Name::new("MainCamera"))
        .insert(Camera{});
}

const BIKE_SPEED:f32 = 40.0;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    //light
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 11.6, -30.1), //z -15
            rotation: Quat::from_rotation_x(-std::f32::consts::PI * 0.45), //-FRAC_PI_4
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
    let platform_length = 200.0;

    let texture_repeat = 8.0;
    let mut uvs = Vec::new();
    uvs.push([0.0, texture_repeat]);
    uvs.push([0.0, 0.0]);
    uvs.push([texture_repeat, 0.0]);
    uvs.push([texture_repeat, texture_repeat]);

    let mut mesh = Mesh::from(shape::Quad::new(Vec2::new(platform_length,
                                                                          platform_length)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);


    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add( StandardMaterial{
                //base_color: Color::rgb(0.0, 0.0, 0.2),
                base_color_texture: Some(texture_handle.clone()),
                double_sided: true,
                ..Default::default()
            }),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0,0.0),
                rotation: Quat::from_rotation_x(-PI/2.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(Sleeping::disabled())
        .insert(Damping { linear_damping: 0.0, angular_damping: 0.0 })
        .insert(Collider::cuboid(
            platform_length/2.0,
            platform_length/2.0,
            0.1
        ));

    //walls
    let wall_height = 16.0;
    let wall_width = 0.8;

    let wall_position = vec![
                                Vec3::new(0.0,wall_height/2.0,platform_length/2.0),
                                Vec3::new(0.0,wall_height/2.0,-platform_length/2.0),
                                Vec3::new(platform_length/2.0,wall_height/2.0,0.0),
                                Vec3::new(-platform_length/2.0,wall_height/2.0,0.0),
                        ];
    let wall_size = vec![
                                Vec3::new(platform_length,wall_height,wall_width),
                                Vec3::new(platform_length,wall_height,wall_width),
                                Vec3::new(wall_width,wall_height,platform_length),
                                Vec3::new(wall_width,wall_height,platform_length)
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
                wall_size[i].x/2.0,
                wall_size[i].y/2.0,
                wall_size[i].z/2.0
            ));
    }

    //bike
    commands.spawn_bundle(SceneBundle {
        scene: asset_server.load("models/bike.glb#Scene0"),
        transform:Transform::from_translation(Vec3::new(0.0,1.0,0.0)),
        ..Default::default()
    })
        .insert(RigidBody::Dynamic)
        .insert(Velocity {
            linvel: Vec3::new(0.0, 0.0, BIKE_SPEED),
            ..default()
        })
        .insert(Damping { linear_damping: -0.1, angular_damping: 0.0 })
        .insert(Collider::cuboid(0.8,
                                 1.0,
                                 2.6))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Name::new("Bike"))
        .insert(Bike{
            direction:Direction::forward
        });
}

const CAMERA_DIFF:Vec3 = Vec3::new(1.11, 40.0, -36.57); //y 16
const CAMERA_LIMIT:f32 = 100.0;

fn move_camera(
    mut query: Query<&mut Transform, (With<Camera>, Without<Bike>)>,
    query_bike: Query<&Transform, With<Bike>>
){
    let bike_transform = query_bike.single();

    for mut transform in query.iter_mut(){

        let new_transform = Transform::from_translation((bike_transform.translation + CAMERA_DIFF).clamp(Vec3::new(-CAMERA_LIMIT,-10.0,-CAMERA_LIMIT),
                                                                                                         Vec3::new(CAMERA_LIMIT,1000.0,CAMERA_LIMIT))).looking_at(bike_transform.translation, Vec3::Y);
        transform.translation = new_transform.translation;
        transform.rotation = new_transform.rotation;
    }
}

fn move_bike(
    keyboard_input:Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Transform, &mut Bike)>
){
    let (mut velocity, mut transform, mut bike) = query.single_mut();

    match bike.direction {
        Direction::forward => {
            if keyboard_input.just_pressed(KeyCode::Right){
                transform.rotate_y(-PI/2.0);
                velocity.linvel =  Vec3::new(-BIKE_SPEED, 0.0, 0.0);
                bike.direction = Direction::right;
            }
            if keyboard_input.just_pressed(KeyCode::Left){
                transform.rotate_y(PI/2.0);
                velocity.linvel =  Vec3::new(BIKE_SPEED, 0.0, 0.0);
                bike.direction = Direction::left;
            }
        }
        Direction::right => {
            if keyboard_input.just_pressed(KeyCode::Down){
                transform.rotate_y(-PI/2.0);
                velocity.linvel =  Vec3::new(0.0, 0.0, -BIKE_SPEED);
                bike.direction = Direction::back;
            }
            if keyboard_input.just_pressed(KeyCode::Up){
                transform.rotate_y(PI/2.0);
                velocity.linvel =  Vec3::new(0.0, 0.0, BIKE_SPEED);
                bike.direction = Direction::forward;
            }
        }
        Direction::left => {
            if keyboard_input.just_pressed(KeyCode::Down){
                transform.rotate_y(PI/2.0);
                velocity.linvel =  Vec3::new(0.0, 0.0, -BIKE_SPEED);
                bike.direction = Direction::back;
            }
            if keyboard_input.just_pressed(KeyCode::Up){
                transform.rotate_y(-PI/2.0);
                velocity.linvel =  Vec3::new(0.0, 0.0, BIKE_SPEED);
                bike.direction = Direction::forward;
            }
        }
        Direction::back => {
            if keyboard_input.just_pressed(KeyCode::Right){
                transform.rotate_y(PI/2.0);
                velocity.linvel =  Vec3::new(-BIKE_SPEED, 0.0, 0.0);
                bike.direction = Direction::right;
            }
            if keyboard_input.just_pressed(KeyCode::Left){
                transform.rotate_y(-PI/2.0);
                velocity.linvel =  Vec3::new(BIKE_SPEED, 0.0, 0.0);
                bike.direction = Direction::left;
            }
        }
    }

}

