use crate::{AppState, prelude::CityAssets};

use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};


pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {

        AssetLoader::new(AppState::Loading, AppState::Menu)
            .with_collection::<FontAssets>()
            .with_collection::<CityAssets>()
            .build(app);
    }
}


#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
    #[asset(path = "fonts/FiraMono-Medium.ttf")]
    pub fira_mono: Handle<Font>,
}
