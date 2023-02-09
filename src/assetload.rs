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

pub const FONTS_AMOUNT: u32 = 13;

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
    #[asset(key = "cheepicus")]
    pub cheepicus: Handle<Image>,
    #[asset(key = "jdpage")]
    pub jdpage: Handle<Image>,
    #[asset(key = "ln")]
    pub ln: Handle<Image>,
    #[asset(key = "nightmare")]
    pub lordnightmare: Handle<Image>,
    #[asset(key = "pastiche")]
    pub pastiche: Handle<Image>,
    #[asset(key = "potash")]
    pub potash: Handle<Image>,
    #[asset(key = "rde")]
    pub rde: Handle<Image>,
    #[asset(key = "yayo")]
    pub yayo: Handle<Image>,
    #[asset(key = "zaratustra")]
    pub zaratustra: Handle<Image>,
}
