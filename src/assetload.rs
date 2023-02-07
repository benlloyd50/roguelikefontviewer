use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::AppState;
pub struct AssetLoadPlugin;

impl Plugin for AssetLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::AssetLoading)
                .continue_to_state(AppState::GameLoading)
                .with_dynamic_collections::<StandardDynamicAssetCollection>(vec![
                    "dynamic_asset.assets",
                ])
                .with_collection::<SpriteAssets>(),
        );
    }
}

pub const MAX_FONT_POSITION: u32 = 4;

#[derive(AssetCollection, Resource)]
pub struct SpriteAssets {
    #[asset(key = "acorn")]
    pub acorn: Handle<Image>,
    #[asset(key = "anikki")]
    pub anikki: Handle<Image>,
    #[asset(key = "cgathick")]
    pub cgathick: Handle<Image>,
    #[asset(key = "cgathin")]
    pub cgathin: Handle<Image>,
}
