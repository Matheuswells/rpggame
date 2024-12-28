use bevy::prelude::*;

pub struct ResolutionPlugin;

impl Plugin for ResolutionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup_resolution);
    }
}

#[derive(Resource)]
pub struct Resolution {
    pub screen_dimensions: Vec2,
    pub pixel_ratio: u32,
    pub frame_size: UVec2,
    pub map_translation: Vec2,
}

fn setup_resolution(mut commands: Commands,window_query: Query<&Window>){
    let window = window_query.single();

    let pixel_ratio = 4;
    let frame_size_x = 16;
    let map_translation_x = pixel_ratio * frame_size_x;

    commands.insert_resource(Resolution {
        screen_dimensions: Vec2::new(window.width(), window.height()),
        pixel_ratio,
        frame_size: UVec2::new(frame_size_x, frame_size_x),
        map_translation: Vec2::new(map_translation_x as f32, map_translation_x as f32 * -1.0),
    });

}