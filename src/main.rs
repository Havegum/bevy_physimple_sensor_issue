use bevy::prelude::*;
use bevy_physimple::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(Physics2dPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_mob)
        .add_system(apply_velocity)
        .add_system(hitbox_system)
        .add_system(player_move_input)
        .run();
}

#[derive(Component)]
struct Controllable;

#[derive(Component)]
struct Acceleration(f32);

#[derive(Component)]
struct HitBox;

/// Top-down orthographic camera at 1/50 scale.
fn setup_camera(mut commands: Commands) {
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale = 1. / 50.;
    commands.spawn_bundle(camera_bundle);
}

/// Spawns controllable sprite, with a larger square sensor attached.
fn spawn_player(mut commands: Commands) {
    let sensor_size = Vec2::new(3., 3.);
    let sendor_sprite_bundle = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(sensor_size),
            ..Default::default()
        },
        ..Default::default()
    };

    let sensor_bundle = SensorBundle {
        shape: CollisionShape::Square(Square::size(sensor_size)),
        ..Default::default()
    };

    let sensor = commands
        .spawn_bundle(sendor_sprite_bundle)
        .insert_bundle(sensor_bundle)
        .insert(HitBox)
        .id();

    let sprite_bundle = SpriteBundle {
        sprite: Sprite {
            color: Color::hsl(200., 0.95, 0.5),
            custom_size: Some(Vec2::splat(1.)),
            ..Default::default()
        },
        ..Default::default()
    };

    commands
        .spawn()
        .insert(Controllable)
        .insert_bundle(sprite_bundle)
        .insert(Acceleration(0.4))
        .insert(Vel::default())
        .add_child(sensor);
}

/// Spawns a single red mob with square collision shape.
fn spawn_mob(mut commands: Commands) {
    let mob_size = Vec2::splat(1.);

    let sprite = SpriteBundle {
        sprite: Sprite {
            color: Color::hsl(9., 0.75, 0.55),
            custom_size: Some(mob_size),
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(3., 0., 0.)),
        ..Default::default()
    };

    let kinematic_bundle = KinematicBundle {
        shape: CollisionShape::Square(Square::size(mob_size)),
        ..Default::default()
    };

    commands
        .spawn()
        .insert_bundle(kinematic_bundle)
        .insert_bundle(sprite);
}

// Colors a sprite yellow sensor detects something, otherwise light gray.
fn hitbox_system(mut q: Query<(&Sensor, &mut Sprite), With<HitBox>>) {
    for (sensor, mut sprite) in q.iter_mut() {
        sprite.color = if sensor.bodies.len() == 0 {
            Color::hsla(0., 0., 1., 0.05)
        } else {
            Color::hsla(60.0, 0.5, 0.5, 0.5)
        }
    }
}

/// Applies velocity to transform, then applies bad friction
fn apply_velocity(mut velocity_query: Query<(&mut Transform, &mut Vel)>, time: Res<Time>) {
    for (mut transform, mut velocity) in velocity_query.iter_mut() {
        transform.translation += velocity.0.extend(0.);
        velocity.0 *= 1. - (time.delta_seconds() * 10.); // bad friction
    }
}

/// Handles keyboard input as movement.
fn player_move_input(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(&mut Vel, &Acceleration), With<Controllable>>,
) {
    let mut direction = Vec2::ZERO;
    if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
        direction.y += 1.;
    }
    if keys.any_pressed([KeyCode::Down, KeyCode::S]) {
        direction.y -= 1.;
    }
    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        direction.x += 1.;
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        direction.x -= 1.;
    }
    if direction == Vec2::ZERO {
        return;
    }

    let direction = direction.normalize() * time.delta_seconds();
    for (mut velocity, acc) in player_query.iter_mut() {
        velocity.0 += direction * acc.0;
    }
}
