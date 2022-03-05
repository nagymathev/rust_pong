use bevy::prelude::*;

const PLAYER_ONE_COLOR: Color = Color::rgb(0.0, 0.0, 1.0);
const PLAYER_TWO_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);
const BALL_COLOR: Color = Color::rgb(1.0 ,1.0, 1.0);

#[derive(Component)]
struct PlayerOne;

#[derive(Component)]
struct PlayerTwo;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Velocity {
    pub velocity: Vec3,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_player_one(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: PLAYER_ONE_COLOR,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(-350.0, 0.0, 0.0),
                scale: Vec3::new(10.0, 100.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
    })
    .insert(Player)
    .insert(Velocity{velocity: Vec3::new(0.0, 0.0, 0.0)})
    .insert(PlayerOne);
}

fn spawn_player_two(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: PLAYER_TWO_COLOR,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(350.0, 0.0, 0.0),
                scale: Vec3::new(10.0, 100.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
    })
    .insert(Player)
    .insert(Velocity{velocity: Vec3::new(0.0, 0.0, 0.0)})
    .insert(PlayerTwo);
}

fn spawn_ball(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: BALL_COLOR,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(50.0, 50.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
    })
    .insert(Ball)
    .insert(Velocity { velocity: Vec3::new(2.0, 0.0, 0.0)});
}

fn move_player_one(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<PlayerOne>>,
) {
    let mut delta_y = 0.0;

    if keyboard_input.pressed(KeyCode::W) {
        delta_y = 5.0;
    }

    if keyboard_input.pressed(KeyCode::S) {
        delta_y = -5.0;
    }


    for mut velocity in query.iter_mut() {
        velocity.velocity.y = delta_y;
    }
}

fn move_player_two(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<PlayerTwo>>,
) {
    let mut delta_y = 0.0;

    if keyboard_input.pressed(KeyCode::Up) {
        delta_y = 5.0;
    }

    if keyboard_input.pressed(KeyCode::Down) {
        delta_y = -5.0;
    }


    for mut velocity in query.iter_mut() {
        velocity.velocity.y = delta_y;
    }
}

fn move_transform(mut query: Query<(&mut Velocity, &mut Transform)>) {
    for (mut velocity, mut transform) in query.iter_mut() {
        if velocity.velocity.x.abs() > 100.0 {
            velocity.velocity.x = velocity.velocity.x.signum() * 100.0;
        }
        if velocity.velocity.y.abs() > 100.0 {
            velocity.velocity.y = velocity.velocity.y.signum() * 100.0;
        }
        transform.translation += velocity.velocity;
    }
}

fn keep_in_screen(mut query: Query<(&mut Velocity, &Transform)>) {
    for (mut velocity, transform) in query.iter_mut() {
        if transform.translation.x.abs() > 400.0 {
            velocity.velocity.x = velocity.velocity.x.abs() * -transform.translation.x.signum();
        }
        if transform.translation.y.abs() > 300.0 {
            velocity.velocity.y = velocity.velocity.y.abs() * -transform.translation.y.signum();
        }
    }
}

fn rectangle_collision(pos0_from: Vec3, pos1_to: Vec3, size0: Vec3, size1: Vec3) -> bool {
    let differnce = pos1_to - pos0_from;

    if (size0.x / 2.0 + size1.x / 2.0) < differnce.x.abs() {
        return false
    }
    if (size0.y / 2.0 + size1.y / 2.0) < differnce.y.abs() {
        return false
    }
    return true;
}

fn ball_collision(
    // mut ballquery: Query<(&Transform, &mut Velocity), With<Ball>>,
    // playerquery: Query<(&Transform, &Velocity), With<Player>>,
    mut query: Query<(&Transform, &mut Velocity, Option<&Player>)>,
    // queryset: QuerySet<(Query<(&Transform, &mut Velocity),
    // With<Ball>>, Query<(&Transform, &Velocity), With<Player>>)>
) {
    let mut iter = query.iter_combinations_mut();
    while let Some([(transform1, mut velocity1, player1), (transform2, mut velocity2, player2)]) =
        iter.fetch_next()
    {
        if let Some(player1) = player1 {
            if let Some(player2) = player2 {
                continue;
            }
            else {
                if rectangle_collision(
                    transform2.translation,
                        transform1.translation,
                        transform2.scale,
                        transform1.scale,
                    ) {
                        velocity2.velocity.x *= -1.1;
                        velocity2.velocity.y += velocity1.velocity.y
                    }
            }
        }
        else {
            if rectangle_collision(
                transform1.translation,
                    transform2.translation,
                    transform1.scale,
                    transform2.scale,
                ) {
                    velocity1.velocity.x *= -1.1;
                    velocity1.velocity.y += velocity2.velocity.y
                }
        }
    }

    // for (playertransform, playervelocity) in playerquery.iter() {
    //     for (balltransform, mut ballvelocity) in ballquery.iter_mut() {
    //         if rectangle_collision(
    //             balltransform.translation,
    //              playertransform.translation,
    //               balltransform.scale,
    //                playertransform.scale,
    //             ) {
    //                 ballvelocity.velocity.x *= -1.1;
    //                 ballvelocity.velocity.y += playervelocity.velocity.y
    //             }
    //     }
    
    
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Pong By Me!".to_string(),
            width: 800.0,
            height: 600.0,
            ..Default::default()
        })
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_player_one)
        .add_startup_system(spawn_player_two)
        .add_startup_system(spawn_ball)
        .add_system(move_player_one)
        .add_system(move_player_two)
        .add_system(keep_in_screen)
        .add_system(ball_collision)
        .add_system(move_transform)
        .add_plugins(DefaultPlugins)
        .run()
}
