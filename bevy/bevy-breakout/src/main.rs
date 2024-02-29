use bevy::{
    asset::AssetMetaCheck,
    audio::Volume,
    math::*,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

const PADDLE_START_Y: f32 = BOTTOM_WALL + 60.0;
const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const PADDLE_SPEED: f32 = 500.0;

const BALL_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const BALL_SPEED: f32 = 400.0;
const BALL_INITIAL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

const LEFT_WALL: f32 = -450.0;
const RIGHT_WALL: f32 = 450.0;
const BOTTOM_WALL: f32 = -300.0;
const TOP_WALL: f32 = 300.0;

const WALL_THICKNESS: f32 = 10.0;
const WALL_BLOCK_WIDTH: f32 = RIGHT_WALL - LEFT_WALL;
const WALL_BLOCK_HEIGHT: f32 = TOP_WALL - BOTTOM_WALL;
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

const BRICK_SIZE: Vec2 = Vec2::new(100.0, 30.0);
const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 270.0;
const GAP_BETWEEN_BRICKS: f32 = 5.0;
const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
const GAP_BETWEEN_BRICKS_AND_SIDES: f32 = 20.0;

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);

const TOP_WALL_ID: &str = "TOP_WALL";
const BOTTOM_WALL_ID: &str = "BOTTOM_WALL";
const LEFT_WALL_ID: &str = "LEFT_WALL";
const RIGHT_WALL_ID: &str = "RIGHT_WALL";

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "__wasm__breakout".to_string(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        canvas: Some("#game-wrapper-canvas".to_string()),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .init_resource::<GameAssets>()
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(Scoreboard { score: 0 })
        .add_systems(Update, (bevy::window::close_on_esc, update_scoreboard))
        .add_systems(Startup, (load_assets, setup).chain())
        .add_systems(
            FixedUpdate,
            (
                move_paddle,
                apply_velocity,
                check_ball_collisions.after(apply_velocity),
            ),
        )
        .run();
}

#[derive(Resource, Debug, Default)]
pub struct GameAssets {
    pub collision_sound: Handle<AudioSource>,
    pub circle: Handle<Image>,
}

fn load_assets(mut game_assets: ResMut<GameAssets>, asset_server: Res<AssetServer>) {
    *game_assets = GameAssets {
        collision_sound: asset_server.load("sounds/breakout_collision.ogg"),
        circle: asset_server.load("textures/circle.png"),
    }
}

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball {
    size: Vec2,
}

#[derive(Component)]
struct Brick {
    health: i8,
}

#[derive(Resource, Clone, Copy)]
struct Scoreboard {
    score: usize,
}

#[derive(Resource, Default, Deref, DerefMut)]
struct CollisionSound(Handle<AudioSource>);

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component, Debug)]
struct Collider {
    size: Vec2,
}

#[derive(Component, Debug)]
struct WallId<'a> {
    value: &'a str,
}

#[derive(Bundle)]
struct WallBundle {
    id: WallId<'static>,
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

fn generate_bricks(commands: &mut Commands) {
    let offset_x = LEFT_WALL + GAP_BETWEEN_BRICKS_AND_SIDES + BRICK_SIZE.x * 0.5;
    let offset_y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_BRICKS + BRICK_SIZE.y * 0.5;

    let bricks_total_width = (RIGHT_WALL - LEFT_WALL) - 2.0 * GAP_BETWEEN_BRICKS_AND_SIDES;
    let bricks_total_height =
        (TOP_WALL - BOTTOM_WALL) - GAP_BETWEEN_BRICKS_AND_CEILING - GAP_BETWEEN_PADDLE_AND_BRICKS;

    let rows = (bricks_total_height / (BRICK_SIZE.y + GAP_BETWEEN_BRICKS)).floor() as i32;
    let columns = (bricks_total_width / (BRICK_SIZE.x + GAP_BETWEEN_BRICKS)).floor() as i32;

    for row in 0..rows {
        for column in 0..columns {
            let brick_pos = vec2(
                offset_x + column as f32 * (BRICK_SIZE.x + GAP_BETWEEN_BRICKS),
                offset_y + row as f32 * (BRICK_SIZE.y + GAP_BETWEEN_BRICKS),
            );

            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: brick_pos.extend(0.0),
                        ..default()
                    },
                    sprite: Sprite {
                        color: BRICK_COLOR,
                        custom_size: Some(BRICK_SIZE),
                        ..default()
                    },
                    ..default()
                },
                Brick { health: 100 },
                Collider { size: BRICK_SIZE },
            ));
        }
    }
}

fn setup(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn(Camera2dBundle::default());

    // let ball_collision_sound = game_assets.collision_sound.clone();
    // commands.insert_resource(CollisionSound(ball_collision_sound));

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: vec3(0.0, PADDLE_START_Y, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                custom_size: Some(PADDLE_SIZE),
                ..default()
            },
            ..default()
        },
        Paddle,
        Collider { size: PADDLE_SIZE },
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: BALL_STARTING_POSITION,
                ..default()
            },
            sprite: Sprite {
                color: BALL_COLOR,
                custom_size: Some(BALL_SIZE),
                ..default()
            },
            texture: game_assets.circle.clone(),
            ..default()
        },
        Ball { size: BALL_SIZE },
        Velocity(BALL_SPEED * BALL_INITIAL_DIRECTION),
    ));

    // walls
    {
        let vertical_wall_size = vec2(WALL_THICKNESS, WALL_BLOCK_HEIGHT + WALL_THICKNESS);
        let horizontal_wall_size = vec2(WALL_BLOCK_WIDTH + WALL_THICKNESS, WALL_THICKNESS);

        //left wall
        commands.spawn(WallBundle {
            id: WallId {
                value: LEFT_WALL_ID,
            },
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: vec3(LEFT_WALL, 0.0, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(vertical_wall_size),
                    ..default()
                },
                ..default()
            },
            collider: Collider {
                size: vertical_wall_size,
            },
        });

        //right wall
        commands.spawn(WallBundle {
            id: WallId {
                value: RIGHT_WALL_ID,
            },
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: vec3(RIGHT_WALL, 0.0, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(vertical_wall_size),
                    ..default()
                },
                ..default()
            },
            collider: Collider {
                size: vertical_wall_size,
            },
        });

        //bottom wall
        commands.spawn(WallBundle {
            id: WallId {
                value: BOTTOM_WALL_ID,
            },
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: vec3(0.0, BOTTOM_WALL, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(horizontal_wall_size),
                    ..default()
                },
                ..default()
            },
            collider: Collider {
                size: horizontal_wall_size,
            },
        });

        //top wall
        commands.spawn(WallBundle {
            id: WallId { value: TOP_WALL_ID },
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: vec3(0.0, TOP_WALL, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    custom_size: Some(horizontal_wall_size),
                    ..default()
                },
                ..default()
            },
            collider: Collider {
                size: horizontal_wall_size,
            },
        });
    }

    // bricks
    generate_bricks(&mut commands);

    // scoreboard
    commands.spawn((TextBundle::from_sections([
        TextSection::new(
            "Score",
            TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: TEXT_COLOR,
                ..default()
            },
        ),
        TextSection::from_style(TextStyle {
            font_size: SCOREBOARD_FONT_SIZE,
            color: SCORE_COLOR,
            ..default()
        }),
    ])
    .with_style(Style {
        position_type: PositionType::Absolute,
        top: SCOREBOARD_TEXT_PADDING,
        left: SCOREBOARD_TEXT_PADDING,
        ..default()
    }),));
}

fn move_paddle(
    input: Res<Input<KeyCode>>,
    time_step: Res<Time<Fixed>>,
    mut query: Query<&mut Transform, With<Paddle>>,
) {
    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;

    if input.pressed(KeyCode::A) {
        direction -= 1.0;
    }

    if input.pressed(KeyCode::D) {
        direction += 1.0;
    }

    let mut new_x =
        paddle_transform.translation.x + direction * PADDLE_SPEED * time_step.delta_seconds();

    new_x = new_x.min(RIGHT_WALL - (WALL_THICKNESS + PADDLE_SIZE.x) * 0.5);
    new_x = new_x.max(LEFT_WALL + (WALL_THICKNESS + PADDLE_SIZE.x) * 0.5);

    paddle_transform.translation.x = new_x;
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<Time<Fixed>>) {
    let dt = time_step.delta_seconds();
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * dt;
        transform.translation.y += velocity.y * dt;
    }
}

fn check_ball_collisions(
    mut commands: Commands,
    mut score: ResMut<Scoreboard>,
    game_assets: Res<GameAssets>,
    bricks_query: Query<Entity, With<Brick>>,
    mut ball_query: Query<(&mut Velocity, &Transform, &Ball)>,
    mut collider_query: Query<(
        Entity,
        &Transform,
        &Collider,
        Option<&WallId<'static>>,
        Option<&mut Brick>,
    )>,
) {
    for (mut ball_velocity, ball_transform, ball) in &mut ball_query {
        for (other_entity, transform, other, opt_wall, opt_brick) in &mut collider_query {
            let collision = collide(
                ball_transform.translation,
                ball.size,
                transform.translation,
                other.size,
            );

            let mut reflect_x = false;
            let mut reflect_y = false;

            if let Some(collision) = collision {
                match collision {
                    Collision::Left => reflect_x = ball_velocity.x > 0.0,
                    Collision::Right => reflect_x = ball_velocity.x < 0.0,
                    Collision::Top => reflect_y = ball_velocity.y < 0.0,
                    Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                    Collision::Inside => {}
                }

                if reflect_x {
                    ball_velocity.x *= -1.;
                }

                if reflect_y {
                    ball_velocity.y *= -1.;
                }

                if let Some(wall_id) = opt_wall {
                    if wall_id.value == BOTTOM_WALL_ID {
                        score.score = 0;

                        for brick_entity in bricks_query.iter() {
                            commands.entity(brick_entity).despawn();
                        }

                        generate_bricks(&mut commands);
                    }
                }

                if let Some(mut brick) = opt_brick {
                    score.score += 1;
                    brick.health = (brick.health - 50).max(0);

                    if brick.health <= 0 {
                        commands.entity(other_entity).despawn();
                    }
                }

                commands.spawn(AudioBundle {
                    source: game_assets.collision_sound.clone(),
                    settings: PlaybackSettings::ONCE.with_volume(Volume::new_relative(0.5)),
                });
            }
        }
    }
}

fn update_scoreboard(score: Res<Scoreboard>, mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[1].value = score.score.to_string();
}
