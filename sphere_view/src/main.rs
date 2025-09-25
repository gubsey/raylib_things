use std::{
    any::Any,
    f32::consts::{FRAC_PI_2, PI},
    fmt::Debug,
};

use bevy::{prelude::*, render::camera::Viewport, window::WindowResolution};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1200., 800.),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, fov_zoom)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-FRAC_PI_2)).with_translation(Vec3 {
            x: 0.,
            y: -1.,
            z: 0.,
        }),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.))),
        MeshMaterial3d(materials.add(Color::srgb_u8(0, 255, 0))),
        Transform::from_xyz(0., 0., 0.),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(4., 8., 4.),
    ));

    commands.spawn((
        MainCam,
        Camera3d::default(),
        Projection::from(PerspectiveProjection::default()),
        Transform::from_xyz(-2.5, 4.5, 9.).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        ViewportCam,
        Camera3d::default(),
        Projection::from(PerspectiveProjection {
            fov: 77. / 360. * PI,
            ..Default::default()
        }),
        Transform::default().looking_at(Vec3::ZERO, Vec3::Y),
        Camera {
            viewport: Some(Viewport {
                physical_position: uvec2(10, 10),
                physical_size: uvec2(300, 300),
                ..Default::default()
            }),
            order: 2,
            ..Default::default()
        },
    ));
}

#[derive(Component)]
struct MainCam;

#[derive(Component)]
struct ViewportCam;

fn fov_zoom(
    mut main_cam: Query<(&mut Projection, &mut Transform), (With<MainCam>, Without<ViewportCam>)>,
    viewport_cam: Query<(&mut Projection, &mut Transform), (With<ViewportCam>, Without<MainCam>)>,
    time: Res<Time>,
) {
    let d = 1.1 + time.elapsed_secs().tan().abs();
    let td = ((d * d) - 1.).sqrt();
    let ty = (1. * td) / d;
    let tx = (1. - (ty * ty)).sqrt();
    let m = ty / (tx - d);
    let c = -(m * 1.1).atan() * 2.;
    let (pro, tran) = main_cam.single_mut().unwrap();
    match pro.into_inner() {
        Projection::Perspective(p) => p.fov = c,
        _ => (),
    };
    let ti = tran.into_inner();
    let norm = ti.translation.normalize();
    ti.translation = norm * d;

    let mut vc = viewport_cam.single_inner().unwrap().1;
    vc.translation = norm * d;
    vc.look_at(Vec3::ZERO, Vec3::Y);
}
