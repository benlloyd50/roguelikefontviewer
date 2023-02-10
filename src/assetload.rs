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

#[derive(AssetCollection, Resource)]
pub struct SpriteAssets {
    #[asset(key = "acorn")]
    pub acorn: Handle<Image>,
    #[asset(key = "alloy")]
    pub alloy: Handle<Image>,
    #[asset(key = "anikki")]
    pub anikki: Handle<Image>,
    #[asset(key = "anikkisquare")]
    pub anikkisquare: Handle<Image>,
    #[asset(key = "buddy")]
    pub buddy: Handle<Image>,
    #[asset(key = "buddygraphical")]
    pub buddygraphical: Handle<Image>,
    #[asset(key = "cgathick")]
    pub cgathick: Handle<Image>,
    #[asset(key = "cgathin")]
    pub cgathin: Handle<Image>,
    #[asset(key = "cheepicus8")]
    pub cheepicus8: Handle<Image>,
    #[asset(key = "cheepicus12")]
    pub cheepicus12: Handle<Image>,
    #[asset(key = "cheepicus14")]
    pub cheepicus14: Handle<Image>,
    #[asset(key = "cheepicus15")]
    pub cheepicus15: Handle<Image>,
    #[asset(key = "curses")]
    pub curses: Handle<Image>,
    #[asset(key = "db")]
    pub db: Handle<Image>,
    #[asset(key = "ddw")]
    pub ddw: Handle<Image>,
    #[asset(key = "dullard")]
    pub dullard: Handle<Image>,
    #[asset(key = "geti")]
    pub geti: Handle<Image>,
    #[asset(key = "haberdash")]
    pub haberdash: Handle<Image>,
    #[asset(key = "herrbdog7")]
    pub herrbdog7: Handle<Image>,
    #[asset(key = "herrbdog12")]
    pub herrbdog12: Handle<Image>,
    #[asset(key = "jdpage")]
    pub jdpage: Handle<Image>,
    #[asset(key = "kein")]
    pub kein: Handle<Image>,
    #[asset(key = "kren")]
    pub kren: Handle<Image>,
    #[asset(key = "ln")]
    pub ln: Handle<Image>,
    #[asset(key = "nightmare")]
    pub lordnightmare: Handle<Image>,
    #[asset(key = "markvii")]
    pub markvii: Handle<Image>,
    #[asset(key = "mkv")]
    pub mkv: Handle<Image>,
    #[asset(key = "mkvsolid")]
    pub mkvsolid: Handle<Image>,
    #[asset(key = "nice")]
    pub nice: Handle<Image>,
    #[asset(key = "nobbins")]
    pub nobbins: Handle<Image>,
    #[asset(key = "nostalgia")]
    pub nostalgia: Handle<Image>,
    #[asset(key = "pastiche")]
    pub pastiche: Handle<Image>,
    #[asset(key = "paul")]
    pub paul: Handle<Image>,
    #[asset(key = "potash8")]
    pub potash8: Handle<Image>,
    #[asset(key = "potash10")]
    pub potash10: Handle<Image>,
    #[asset(key = "rde")]
    pub rde: Handle<Image>,
    #[asset(key = "smoothwalls")]
    pub smoothwalls: Handle<Image>,
    #[asset(key = "taffer")]
    pub taffer: Handle<Image>,
    #[asset(key = "talryth")]
    pub talryth: Handle<Image>,
    #[asset(key = "terbert7")]
    pub terbert7: Handle<Image>,
    #[asset(key = "terbert10")]
    pub terbert10: Handle<Image>,
    #[asset(key = "terminus")]
    pub terminus: Handle<Image>,
    #[asset(key = "tilesetunknown")]
    pub tilesetunknown: Handle<Image>,
    #[asset(key = "tocky")]
    pub tocky: Handle<Image>,
    #[asset(key = "unknown")]
    pub unknown: Handle<Image>,
    #[asset(key = "vidumec")]
    pub vidumec: Handle<Image>,
    #[asset(key = "yayo8")]
    pub yayo8: Handle<Image>,
    #[asset(key = "yayo13")]
    pub yayo13: Handle<Image>,
    #[asset(key = "zaratustra5")]
    pub zaratustra5: Handle<Image>,
    #[asset(key = "zaratustra8")]
    pub zaratustra8: Handle<Image>,
    #[asset(key = "zesty")]
    pub zesty: Handle<Image>,
}
