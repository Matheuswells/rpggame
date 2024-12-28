#![windows_subsystem = "windows"]
pub mod player;
mod game;
mod map;
mod layers;
mod gridselector;
mod resolution;
mod cursor;

use bevy::prelude::*;
use bevy::window::{CursorOptions, WindowMode};
use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
};
use bevy::text::FontSmoothing;

struct OverlayColor;

impl OverlayColor {
    // const RED: Color = Color::srgb(1.0, 0.0, 0.0);
    // const GREEN: Color = Color::srgb(0.0, 1.0, 0.0);
    const WHITE: Color = Color::srgb(1.0, 1.0, 1.0);
}

fn main() {
    App::new().insert_resource(ClearColor(Color::srgba(0.231, 0.502,  0.302, 1.0)))
        .add_plugins(
            (
                FpsOverlayPlugin {
                    config: FpsOverlayConfig {
                        text_config: TextFont {
                            font_size: 10.0,
                            font: default(),
                            font_smoothing: FontSmoothing::default(),
                        },
                        text_color: OverlayColor::WHITE,
                        enabled: true,
                    },
                },
            DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            title: String::from("test"),
                            position: WindowPosition::Centered(MonitorSelection::Primary),
                            resolution: Vec2::new(1920.0, 1080.0).into(),
                            mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                            cursor_options: CursorOptions {
                                visible: false,
                                ..default()
                            },
                            ..Default::default()
                        }),
                        ..Default::default()
                    })
                    .set(ImagePlugin::default_nearest()),
                game::GamePlugin,
            )
        )
        .run();
}

