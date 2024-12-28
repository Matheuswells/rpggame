use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use bevy::prelude::*;
use bevy::reflect::erased_serde::__private::serde::{Deserialize, Serialize};
use bevy::render::view::RenderLayers;
use crate::layers::{MAP_LAYER};
use rand::Rng;
use crate::resolution;

pub struct MapPlugin;

#[derive(Clone, Debug, Serialize, Deserialize)]
enum Collision {
    Full,
    Middle,
    Slow,
    Passable,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
enum MapLayer {
    Base,
    Objects,
    Details,
    Particles,
    Weather
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    tile: usize,
    collision: Collision,
    texture: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Chunk {
    blocks: Vec<Vec<Block>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Map {
    chunks: Vec<Vec<Chunk>>,
}

impl Map {
    pub fn save_to_json(&self, path: &str) -> Result<(), std::io::Error> {
        let json = serde_json::to_string_pretty(self)?;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        file.write_all(json.as_bytes())?;

        Ok(())
    }

    pub fn load_from_json(path: &str) -> Result<Self, std::io::Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let map: Map = serde_json::from_str(&contents)?;

        Ok(map)
    }
}

fn generate_map(size: usize) -> Vec<Vec<usize>> {
    let mut map_data = vec![vec![0; size]; size];

    for i in 0..size {
        map_data[0][i] = 1;
        map_data[size - 1][i] = 23;
        map_data[i][0] = 11;
        map_data[i][size - 1] = 13;
    }
    map_data[0][0] = 41;
    map_data[0][size - 1] = 43;
    map_data[size - 1][0] = 22;
    map_data[size - 1][size - 1] = 24;

    let mut rng = rand::thread_rng();
    for i in 1..size - 1 {
        for j in 1..size - 1 {
            map_data[i][j] = 12;
            if rng.gen_bool(0.1) { // 30% chance to randomize
                map_data[i][j] = rng.gen_range(56..=60);
            }
        }
    }
    map_data
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_map);
    }
}

fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    resolution: Res<resolution::Resolution>,
) {
    let texture: Handle<Image> = asset_server.load("nature/ground/nature_ground.png");
    let frame_size = resolution.frame_size;
    let spacing = UVec2::new(0, 0);
    let offset = UVec2::new(0, 0);
    let layout = TextureAtlasLayout::from_grid(frame_size, 11, 7, Option::from(spacing), Option::from(offset));
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let invert = 1.0;
    let textures: HashMap<String, Handle<Image>> = [
        ("grass".to_string(), texture.clone()),
    ].iter().cloned().collect();

    // let map_data = generate_map(16);
    //
    // let mut blocks = Vec::new();
    // for row in map_data.iter() {
    //     let mut block_row = Vec::new();
    //     for &tile in row.iter() {
    //         block_row.push(Block {
    //             tile,
    //             texture: String::from("grass"),
    //             collision: Collision::Passable,
    //         });
    //     }
    //     blocks.push(block_row);
    // }
    //
    // let chunk = Chunk { blocks };
    // let map = Map { chunks: vec![vec![chunk.clone()]] };
    //
    // map.save_to_json("maps/main.json").unwrap();

    let map = Map::load_from_json("maps/main.json").unwrap();

    let chunk = &map.chunks[0][0];

    for (i, row) in chunk.blocks.iter().enumerate() {
        for (j, block) in row.iter().enumerate() {
            commands.spawn((
                Sprite::from_atlas_image(
                    textures.get(&block.texture).unwrap().clone(),
                    TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: block.tile,
                    },
                ),
                Transform::from_scale(Vec3::new(resolution.pixel_ratio as f32 * invert, resolution.pixel_ratio as f32, 1.0))
                    .with_translation(Vec3::new(j as f32 * resolution.map_translation.x, i as f32 * resolution.map_translation.y, 0.0)),
                RenderLayers::layer(MAP_LAYER),
            ));
        }
    }
}