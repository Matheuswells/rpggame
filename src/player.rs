use std::collections::HashMap;
use std::time::Duration;
use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use crate::layers::{PLAYER_LAYER};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_player)
            .add_systems(Update, (handle_player_movement, get_next_animation, execute_animations, camera_follow_player));
    }
}

fn camera_follow_player(
    player_query: Query<&Transform, With<PlayerSprite>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<PlayerSprite>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    resolution: Res<crate::resolution::Resolution>,
) {
    let texture: Handle<Image> = asset_server.load("female.png");
    let frame_size = UVec2::new(16, 32);
    let spacing = UVec2::new(32, 32);
    let offset = UVec2::new(16, 16);
    let layout = TextureAtlasLayout::from_grid(frame_size, 8, 12, Option::from(spacing), Option::from(offset));
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let animations = HashMap::from([
        (PlayerAnimationState::IdleForward, (24, 31, 7, texture.clone(), texture_atlas_layout.clone(), false)),
        (PlayerAnimationState::IdleLeft, (8, 15, 7, texture.clone(), texture_atlas_layout.clone(), false)),
        (PlayerAnimationState::IdleRight, (40, 47, 7, texture.clone(), texture_atlas_layout.clone(), false)),
        (PlayerAnimationState::Idle, (0, 7, 7, texture.clone(), texture_atlas_layout.clone(), false)),
        (PlayerAnimationState::WalkForward, (72,79, 7, texture.clone(), texture_atlas_layout.clone(), false)),
        (PlayerAnimationState::WalkLeft, (56, 63, 7, texture.clone(), texture_atlas_layout.clone(), false)),
        (PlayerAnimationState::WalkBackward, (48, 55, 7, texture.clone(), texture_atlas_layout.clone(), false)),
        (PlayerAnimationState::WalkRight, (88, 95, 7, texture.clone(), texture_atlas_layout.clone(), false)),
    ]);

    let animation_config = AnimationConfig::new(PlayerAnimationState::Idle, animations);

    let mut invert = 1.0;

    if animation_config.animations[&animation_config.current_state].5{
        invert = -1.0;
    }

    commands.spawn((

        Sprite::from_atlas_image(
            animation_config.animations[&animation_config.current_state].3.clone(),
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_config.animations[&animation_config.current_state].0,
            },
        ),
        Transform::from_scale(Vec3::new(resolution.pixel_ratio as f32 * invert, resolution.pixel_ratio as f32, 1.0)).with_translation(Vec3::new(0.0, 0.0, 100.0)),
        animation_config,
        PlayerSprite,
        RenderLayers::layer(PLAYER_LAYER)
    ));
}

#[derive(Component)]
struct AnimationConfig {
    animations: HashMap<PlayerAnimationState, (usize, usize, u8, Handle<Image>, Handle<TextureAtlasLayout>, bool)>,
    current_state: PlayerAnimationState,
    frame_timer: Timer,
}

#[derive(Component)]
struct PlayerSprite;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum PlayerAnimationState {
    Idle,
    IdleForward,
    IdleLeft,
    IdleRight,
    WalkForward,
    WalkLeft,
    WalkRight,
    WalkBackward,
}

impl AnimationConfig {
    fn new(initial_state: PlayerAnimationState, animations: HashMap<PlayerAnimationState, (usize, usize, u8, Handle<Image>, Handle<TextureAtlasLayout>, bool )> ) -> Self {
        Self {
            animations: animations.clone(),
            current_state: initial_state.clone(),
            frame_timer: Self::timer_from_fps(animations[&initial_state].2),
        }
    }

    fn set_state(&mut self, state: PlayerAnimationState) {
        if self.current_state != state {
            self.current_state = state.clone();
            let (_, _, fps, _, _, _) = self.animations[&state];
            self.frame_timer = Self::timer_from_fps(fps);
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }

    fn get_current_frame_range(&self) -> (usize, usize, u8, Handle<Image>, Handle<TextureAtlasLayout>, bool) {
        self.animations[&self.current_state].clone()
    }
}

fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        config.frame_timer.tick(time.delta());

        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                let (first, last, _,_,_,_) = config.get_current_frame_range();
                if atlas.index == last {
                    atlas.index = first;
                } else if atlas.index < first || atlas.index > last {
                    atlas.index = first;
                } else {
                    atlas.index += 1;
                }
                let (_, _, fps,_,_,_) = config.animations[&config.current_state];
                config.frame_timer = AnimationConfig::timer_from_fps(fps);
            }
        }
    }
}

const SPEED: f32 = 300.0;
const MOVEMENT_DELAY: f32 = 0.08;

fn handle_player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &PlayerSprite)>,
    time: Res<Time>,
    mut last_movement: Local<f32>,
    mut is_moving: Local<bool>,
) {
    let current_time = time.elapsed_secs();
    if !should_player_move(current_time, *last_movement, *is_moving) {
        return;
    }

    for (mut transform, _) in query.iter_mut() {
        *is_moving = move_player(&keyboard_input, &mut transform, time.delta_secs());
    }

    *last_movement = current_time;
}

fn should_player_move(current_time: f32, last_movement: f32, is_moving: bool) -> bool {
    current_time - last_movement >= MOVEMENT_DELAY || is_moving
}

fn move_player(keyboard_input: &Res<ButtonInput<KeyCode>>, transform: &mut Transform, delta_secs: f32) -> bool {
    let mut moved = false;

    if keyboard_input.pressed(KeyCode::KeyW) {
        transform.translation.y += SPEED * delta_secs;
        moved = true;
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        transform.translation.x -= SPEED * delta_secs;
        moved = true;
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        transform.translation.y -= SPEED * delta_secs;
        moved = true;
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        transform.translation.x += SPEED * delta_secs;
        moved = true;
    }

    moved
}
fn get_next_animation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut AnimationConfig>,
) {
    for mut animation in query.iter_mut() {
        let new_state = match () {
            _ if keyboard_input.pressed(KeyCode::KeyW) => PlayerAnimationState::WalkForward,
            _ if keyboard_input.pressed(KeyCode::KeyA) => PlayerAnimationState::WalkLeft,
            _ if keyboard_input.pressed(KeyCode::KeyS) => PlayerAnimationState::WalkBackward,
            _ if keyboard_input.pressed(KeyCode::KeyD) => PlayerAnimationState::WalkRight,
            _ => match animation.current_state {
                PlayerAnimationState::WalkForward => PlayerAnimationState::IdleForward,
                PlayerAnimationState::WalkLeft => PlayerAnimationState::IdleLeft,
                PlayerAnimationState::WalkBackward => PlayerAnimationState::Idle,
                PlayerAnimationState::WalkRight => PlayerAnimationState::IdleRight,
                PlayerAnimationState::IdleForward => PlayerAnimationState::IdleForward,
                PlayerAnimationState::IdleLeft => PlayerAnimationState::IdleLeft,
                PlayerAnimationState::IdleRight => PlayerAnimationState::IdleRight,
                _ => animation.current_state.clone(),
            },
        };

        animation.set_state(new_state);
    }
}