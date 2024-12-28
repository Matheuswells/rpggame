use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::window::PrimaryWindow;
use crate::game::MainCamera;
use crate::layers::{CURSOR_LAYER};

pub(crate) struct CursorPlugin;

#[derive(Component)]
struct GameCursor {}
impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, (update_sprite_position_on_resize));
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut q_window: Query<&Window, With<PrimaryWindow>>) {
    let texture = asset_server.load("cursors/normal.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::new(32,32), 1, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let mut spawnpos = Vec3::new(0.0, 0.0, 101.0);

    if let Some(position) = q_window.single().cursor_position() {
        spawnpos = Vec3::new(position.x, position.y, 0.0);
    }

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        ),
        Transform::from_translation(spawnpos),
        RenderLayers::layer(CURSOR_LAYER),
        GameCursor {},
    ));
}



fn update_sprite_position_on_resize(
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut query: Query<(&mut Transform, &Sprite), With<GameCursor>>,
) {
    if let Some(window) = q_window.iter().next() {
        let left_margin = 16.0;
        let top_margin = 16.0;

        let mut mouse_position_x = 0.0;
        let mut mouse_position_y = 0.0;
        if let Some(position) = q_window.single().cursor_position() {
            mouse_position_x = position.x + left_margin;
            mouse_position_y = position.y + top_margin;
        }

        if let Some((camera, camera_transform)) = q_camera.iter().next() {
            let camera_position = camera_transform.translation();

            for (mut transform, _sprite) in query.iter_mut() {

                transform.translation = Vec3::new(
                    camera_position.x - window.width() / 2.0 + mouse_position_x,
                    camera_position.y + window.height() / 2.0 - mouse_position_y,
                    transform.translation.z,
                );
            }
        }
    }
}