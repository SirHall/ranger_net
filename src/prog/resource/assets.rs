use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct ProgAssets {
    // #[asset(texture_atlas_layout(tile_size_x = 96, tile_size_y = 99, columns = 8, rows = 1))]
    #[asset(path = "spr/ground.png")]
    pub ground: Handle<Image>,
    #[asset(path = "spr/bullet.png")]
    pub bullet: Handle<Image>,
}
