use bevy::{prelude::*, window::WindowMode};
use chrono::{Local, Timelike};
use core::f32::consts::PI;
use devcaders::{Button, DevcadeControls, Player};
use rand::{seq::SliceRandom, thread_rng};

#[derive(Component)]
struct Hour {
    number: u8,
    index: u8,
}

#[derive(Component)]
struct MinuteHand;

#[derive(Component)]
struct HourHand;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::Fullscreen,
                ..default()
            }),
            ..default()
        }))
        .add_startup_system(setup)
        .add_systems((shuffle_hours, watch_exit))
        .add_systems(
            (update_text, update_hour, update_minute).in_schedule(CoreSchedule::FixedUpdate),
        )
        .run();
}

fn watch_exit(controls: DevcadeControls) {
    if controls.pressed(Player::P1, Button::Menu) || controls.pressed(Player::P2, Button::Menu) {
        std::process::exit(0);
    }
}

fn shuffle_hours(mut hours: Query<&mut Hour>, controls: DevcadeControls) {
    if !controls.just_pressed(Player::P1, Button::A1)
        && !controls.just_pressed(Player::P2, Button::A1)
    {
        return;
    }
    let mut index = 0;
    let mut new_hours: Vec<u8> = (0..12).collect();
    new_hours.shuffle(&mut thread_rng());

    for mut hour in hours.iter_mut() {
        hour.index = *new_hours.get(index).unwrap();
        index += 1;
    }
}

fn setup(mut commands: Commands, mut windows: Query<&mut Window>, assets: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let window_res = &windows.single_mut().resolution;
    let w = window_res.width() as f32;
    let h = window_res.height() as f32;

    // Background
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0., 1., 1.),
            custom_size: Some(Vec2::new(w, h)),
            ..default()
        },
        ..default()
    });

    // Hours

    let mut hours: Vec<u8> = (1..=12).collect();
    hours.shuffle(&mut thread_rng());

    let font = assets.load("OpenSans.ttf");
    for i in 0..12 {
        let text = Text2dBundle {
            text: Text::from_section(
                format!("{}", if i == 0 { 12 } else { i }),
                TextStyle {
                    font: font.clone(),
                    font_size: 50.,
                    color: Color::BLACK,
                },
            ),
            ..default()
        };

        commands.spawn((
            text,
            Hour {
                index: *hours.get(i).unwrap(),
                number: i as u8,
            },
        ));
    }

    // Hour Hand

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(w / 2. * 0.5, 5.)),
                ..default()
            },
            ..default()
        },
        HourHand,
    ));

    // Minute hand

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLACK,
                custom_size: Some(Vec2::new(w / 2. * 0.7, 5.)),
                ..default()
            },
            ..default()
        },
        MinuteHand,
    ));
}

fn update_text(windows: Query<&Window>, mut hours: Query<(&mut Transform, &mut Hour)>) {
    let window_res = &windows.single().resolution;
    let w = window_res.width() as f32;
    let h = window_res.height() as f32;

    for mut hour in hours.iter_mut() {
        let transform = &mut hour.0;
        let theta = hour.1.index as f32 * PI / 6.;
        transform.translation = Vec3::new(
            theta.cos() * 0.9 * (w / 2.),
            theta.sin() * 0.9 * (h / 2.),
            1.,
        );
    }
}

fn update_hour(
    windows: Query<&mut Window>,
    hours: Query<&Hour>,
    mut minute_hand: Query<(&mut Transform, With<HourHand>)>,
) {
    let window_res = &windows.single().resolution;
    let w = window_res.width() as f32;
    let h = window_res.height() as f32;

    let now = Local::now();
    let hour = now.hour() % 12;
    let minute = now.minute();
    let seconds = now.second();

    let hour_index = hours.iter().find(|h| h.number == hour as u8).unwrap().index;
    let next_hour_index = hours
        .iter()
        .find(|h| h.number == ((hour + 1) % 12) as u8)
        .unwrap()
        .index;

    let hour_theta = (((12 + hour_index - next_hour_index) % 12) as f32
        * (minute as f32 + (seconds as f32 / 60.))
        / -60.
        + hour_index as f32)
        * PI
        / 6.;

    let transform = &mut minute_hand.single_mut().0;

    transform.rotation =
        Quat::from_rotation_z((hour_theta.sin() * (h / w)).atan2(hour_theta.cos()));
    transform.translation = Vec3::new(
        hour_theta.cos() * 0.5 * w / 4.,
        hour_theta.sin() * 0.5 * h / 4.,
        1.,
    );
    transform.scale = Vec3::new(
        hour_theta.sin().powi(2) * (h / w) + hour_theta.cos().powi(2),
        1.,
        1.,
    );
}

fn update_minute(
    windows: Query<&mut Window>,
    hours: Query<&Hour>,
    mut minute_hand: Query<(&mut Transform, With<MinuteHand>)>,
) {
    let window_res = &windows.single().resolution;
    let w = window_res.width() as f32;
    let h = window_res.height() as f32;

    let now = Local::now();
    let minute = now.minute();
    let seconds = now.second();
    let millis = now.timestamp_millis() % 1000;

    let transform = &mut minute_hand.single_mut().0;

    let minute_index = hours
        .iter()
        .find(|&h| h.number == ((minute / 5 + 11) % 12 + 1) as u8)
        .unwrap()
        .index;
    let next_minute_index = hours
        .iter()
        .find(|&h| h.number == ((minute / 5) % 12 + 1) as u8)
        .unwrap()
        .index;

    let minute_theta =
        (((((minute % 5) as f32 + ((seconds as f32 + (millis as f32 / 1000.)) / 60.)) / 5.)
            * ((12 + minute_index - next_minute_index) % 12) as f32)
            * -1.
            + minute_index as f32)
            * PI
            / 6.;

    transform.rotation =
        Quat::from_rotation_z((minute_theta.sin() * (h / w)).atan2(minute_theta.cos()));
    transform.translation = Vec3::new(
        minute_theta.cos() * 0.7 * w / 4.,
        minute_theta.sin() * 0.7 * h / 4.,
        1.,
    );
    transform.scale = Vec3::new(
        minute_theta.sin().powi(2) * (h / w) + minute_theta.cos().powi(2),
        1.,
        1.,
    );
}
