use bevy::prelude::*;
use bevy::render::view::{RenderLayers};
use crate::{cursor, gridselector, map, player, resolution};
use crate::layers::{CURSOR_LAYER, EDITOR_LAYER, MAP_LAYER, PLAYER_LAYER};
pub struct GamePlugin;

#[derive(Component)]
pub struct MainCamera;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(
            (
                player::PlayerPlugin,
                map::MapPlugin,
                gridselector::GridSelectorPlugin,
                resolution::ResolutionPlugin,
                cursor::CursorPlugin,
            )
        ).add_systems(Startup, setup_scene);
    }
}

fn setup_scene(mut commands: Commands) {
    commands.spawn((Camera2d::default(), MainCamera, RenderLayers::from_layers(&[MAP_LAYER,PLAYER_LAYER, CURSOR_LAYER, EDITOR_LAYER])));
}