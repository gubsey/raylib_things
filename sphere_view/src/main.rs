use std::f32::consts::{FRAC_PI_2, PI};

use bevy::{
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    platform::hash::RandomState,
    prelude::*,
    render::{
        camera::Viewport,
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
    window::WindowResolution,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(1400., 900.),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            WireframePlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (fov_zoom, toggle_wireframe, rotate_sphere))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut wireframe_conf: ResMut<WireframeConfig>,
) {
    wireframe_conf.global = true;
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-FRAC_PI_2)).with_translation(Vec3 {
            x: 0.,
            y: -1.,
            z: 0.,
        }),
    ));

    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });

    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.).mesh().uv(100, 100))),
        MeshMaterial3d(debug_material),
        Transform::from_xyz(0., 0., 0.),
        IsSphere,
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

fn toggle_wireframe(
    mut wireframe_config: ResMut<WireframeConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}

fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    // let mut palette: [u8; 32] = [
    //     255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
    //     198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    // ];

    let texture_data = (0..TEXTURE_SIZE * TEXTURE_SIZE * 4)
        .map(|_| rand::random())
        .collect::<Vec<u8>>();
    // for y in 0..TEXTURE_SIZE {
    //     let offset = TEXTURE_SIZE * y * 4;
    //     texture_data[offset..(offset + TEXTURE_SIZE * 4)]
    //         .iter_mut()
    //         .for_each(|x| *x = rand::random());
    //     //palette.rotate_right(4);
    // }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}

#[derive(Component)]
struct IsSphere;

#[derive(Component)]
struct MainCam;

#[derive(Component)]
struct ViewportCam;

fn rotate_sphere(mut sphere: Query<(&mut Transform, &Mesh3d), With<IsSphere>>, time: Res<Time>) {
    sphere
        .single_mut()
        .unwrap()
        .0
        .rotate_y(time.delta_secs() / 10.);
}

fn fov_zoom(
    mut main_cam: Query<(&mut Projection, &mut Transform), (With<MainCam>, Without<ViewportCam>)>,
    viewport_cam: Query<(&mut Projection, &mut Transform), (With<ViewportCam>, Without<MainCam>)>,
    time: Res<Time>,
) {
    let d: f32 = 1.1 + ((time.elapsed_secs() % 14.) - 7.).powf(2.);
    if d.is_nan() {
        return;
    }
    let td = ((d * d) - 1.).sqrt();
    let ty = (1. * td) / d;
    let tx = (1. - (ty * ty)).sqrt();
    let m = ty / (tx - d);
    let c = -(m * 1.1).atan() * 2.;
    if c.is_nan() {
        return;
    }
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
