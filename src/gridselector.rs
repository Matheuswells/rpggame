use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::window::PrimaryWindow;
use crate::game::MainCamera;
use crate::layers::{EDITOR_LAYER};
use crate::resolution;

pub struct GridSelectorPlugin;


#[derive(Component)]
struct MousePosition;

#[derive(Component)]
struct BlockHovered(isize, isize);

#[derive(Component)]
struct GridSelector;

#[derive(Component)]
struct WorldPosition;
#[derive(Resource, Default)]
struct HoveredBlock(isize, isize);

#[derive(Resource, Default)]
struct MyWorldCoords(Vec2);
impl Plugin for GridSelectorPlugin {
    fn build(&self, app: &mut App) {
        println!("GridSelectorPlugin build");
        app
            .insert_resource(MyWorldCoords::default())
            .insert_resource(HoveredBlock::default())
            .add_systems(Startup, (setup, position_selector))
            .add_systems(Update, (cursor_position, get_hovered_block, update_text, update_selector_position, update_world_position));
    }
}


fn setup(
    mut commands: Commands,
) {
    commands.spawn((
        Text::new("CURSOR:"),
        TextFont {
            font_size: 10.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(0.0),
            ..default()
        }
    )).with_child((
        TextSpan::default(),
        TextFont {
            font_size: 10.0,
            ..default()
        },
        MousePosition
    ));

    commands.spawn((
        Text::new("Mouse World Position:"),
        TextFont {
            font_size: 10.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(40.0),
            left: Val::Px(0.0),
            ..default()
        }
    )).with_child((
        TextSpan::default(),
        TextFont {
            font_size: 10.0,
            ..default()
        },
        WorldPosition
    ));

    commands.spawn((
        Text::new("BLOCK:"),
        TextFont {
            font_size: 10.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(60.0),
            left: Val::Px(0.0),
            ..default()
        }
    )).with_child((
        TextSpan::default(),
        TextFont {
            font_size: 10.0,
            ..default()
        },
        BlockHovered(0, 0)
    ));
}

fn position_selector(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    resolution: Res<resolution::Resolution>,
) {
    let texture: Handle<Image> = asset_server.load("gridselector.png");
    let frame_size = resolution.frame_size;
    let spacing = UVec2::new(0, 0);
    let offset = UVec2::new(0, 0);
    let layout = TextureAtlasLayout::from_grid(frame_size, 1, 1, Option::from(spacing), Option::from(offset));
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        Sprite::from_atlas_image(
            texture.clone(),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        ),
        Transform::from_scale(Vec3::new(resolution.pixel_ratio as f32, resolution.pixel_ratio as f32, 1.0)).with_translation(Vec3::new(0.0, 0.0, 100.0)),
        RenderLayers::layer(EDITOR_LAYER),
        GridSelector
    ));
}

fn cursor_position(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut text_query: Query<&mut TextSpan, With<MousePosition>>,
) {
    if let Some(position) = q_windows.single().cursor_position() {
        for mut span in text_query.iter_mut() {
            **span = format!("x: {:?} y: {:?}", position.x, position.y).into();
        }
    }
}

fn calculate_value(world_position: f32) -> f32 {
    let position_correction = 32.0;
    let corrected_position = world_position.abs() - position_correction;
    let multiplier = if corrected_position != 0.0 {
        (corrected_position / 32.0).ceil() as i32 - 1
    } else {
        0
    };
    (multiplier as f32 / 2.0).ceil() * world_position.signum()
}
fn get_hovered_block(
    mut mycoords: ResMut<MyWorldCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut hovered_block: ResMut<HoveredBlock>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    resolution: Res<resolution::Resolution>,
) {
    let (camera, camera_transform) = q_camera.single();

    let window = q_window.single();

    if let Some(cursor) = window.cursor_position() {
        if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor) {
            let world_position = ray.origin.truncate();
            mycoords.0 = world_position;
            let _block_size = (resolution.pixel_ratio as f32 + 1.0) * resolution.frame_size.x as f32;
            let grid_x = ((world_position.x) / (64.0 / 2.0) ) as isize;
            let grid_y = ((world_position.y) / (64.0 / 2.0)) as isize;

            hovered_block.0 = (grid_x as f32 - calculate_value(world_position.x)) as isize;
            hovered_block.1 = (grid_y as f32 - calculate_value(world_position.y)) as isize;
        }
    }
}
fn update_text(
    hovered_block: Res<HoveredBlock>,
    mut text_query: Query<&mut TextSpan, With<BlockHovered>>,
) {
    for mut span in text_query.iter_mut() {
        **span = format!("x: {:?} y: {:?}", hovered_block.0, hovered_block.1).into();
    }
}

fn update_selector_position(
    hovered_block: Res<HoveredBlock>,
    mut q_selector: Query<(&mut Transform, &RenderLayers), With<GridSelector>>,
    resolution: Res<resolution::Resolution>,
) {
    for (mut transform, _) in q_selector.iter_mut() {
        transform.translation.x = hovered_block.0 as f32 * resolution.frame_size.x as f32 * resolution.pixel_ratio as f32;
        transform.translation.y = hovered_block.1 as f32 * resolution.frame_size.x as f32 * resolution.pixel_ratio as f32;
    }
}

fn update_world_position(
    mycoords: Res<MyWorldCoords>,
    mut text_query: Query<&mut TextSpan, With<WorldPosition>>,
) {
    for mut span in text_query.iter_mut() {
        **span = format!("x: {:?} y: {:?}", mycoords.0.x, mycoords.0.y).into();
    }
}