use bevy::{app::AppExit, input::mouse::MouseWheel, prelude::*, window::WindowMode};

#[derive(Component, Default)]
struct SweepCamera {
    target: Vec3,
    to_camera: Vec3,
    distance: f32,

    plan_target: Vec3,
    plan_distance: f32,
}

impl SweepCamera {
    fn new(target: Vec3, to_camera: Vec3, distance: f32) -> Self {
        SweepCamera {
            target,
            to_camera,
            distance,
            plan_target: target,
            plan_distance: distance,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(quit_on_q)
        .add_system(sweep_camera)
        .run();
}

fn setup(
    mut commands: Commands,
    mut windows: Query<&mut Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    windows.single_mut().mode = WindowMode::Fullscreen;

    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn((
        Camera3dBundle::default(),
        SweepCamera::new(Vec3::ZERO, Vec3::new(0.0, 1.0, 1.0).normalize(), 5.0),
    ));
}

fn quit_on_q(keys: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Q) {
        exit.send(AppExit);
    }
}

fn sweep_camera(
    time: Res<Time>,
    mut scroll: EventReader<MouseWheel>,
    mut kb: Res<Input<KeyCode>>,
    mut query: Query<(&mut SweepCamera, &mut Transform)>,
) {
    let scroll: f32 = scroll.iter().map(|ev| ev.y).sum();
    let scroll_base = 1.1f32;
    let scroll_mult = scroll_base.powf(scroll);

    let mut offset = Vec3::new(0.0, 0.0, 0.0);
    let mut moving = false;
    if kb.pressed(KeyCode::Down) {
        offset.z += 1.0;
        moving = true;
    }
    if kb.pressed(KeyCode::Up) {
        offset.z -= 1.0;
        moving = true;
    }
    if kb.pressed(KeyCode::Right) {
        offset.x += 1.0;
        moving = true;
    }
    if kb.pressed(KeyCode::Left) {
        offset.x -= 1.0;
        moving = true;
    }
    offset = if offset.length() > 0.1 {
        offset.normalize()
    } else {
        offset
    };
    offset /= 50.0;

    if let Ok((mut cam, mut transform)) = query.get_single_mut() {
        let lerp_s = time.delta_seconds() * 20.0;

        cam.plan_distance = (cam.plan_distance * scroll_mult).clamp(2.0, 40.0);
        cam.distance += (cam.plan_distance - cam.distance) * lerp_s;

        let tmp = cam.distance;
        cam.plan_target += offset * tmp;
        cam.target = cam.target.lerp(cam.plan_target, lerp_s);

        *transform = Transform::from_translation(cam.target + cam.to_camera * cam.distance)
            .looking_at(cam.target, Vec3::Y);
    }
}
