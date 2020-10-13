use super::{ChannelAssetHandler, LoadRequest};
use crate::{AssetLoadError, AssetLoader, AssetResult, Handle};
use anyhow::Result;
use async_trait::async_trait;
use crossbeam_channel::Sender;

/// Handles load requests from an AssetServer

#[async_trait]
pub trait AssetLoadRequestHandler: Send + Sync + 'static {
    async fn handle_request(&self, load_request: &LoadRequest);
    fn extensions(&self) -> &[&str];
}

impl<TLoader, TAsset> ChannelAssetHandler<TLoader, TAsset>
where
    TLoader: AssetLoader<TAsset>,
{
    pub fn new(loader: TLoader, sender: Sender<AssetResult<TAsset>>) -> Self {
        ChannelAssetHandler { sender, loader }
    }

    fn load_asset(&self, load_request: &LoadRequest) -> Result<TAsset, AssetLoadError> {
        self.loader.load_from_file(&load_request.path)
    }
}

#[async_trait]
impl<TLoader, TAsset> AssetLoadRequestHandler for ChannelAssetHandler<TLoader, TAsset>
where
    TLoader: AssetLoader<TAsset> + 'static,
    TAsset: Send + 'static,
{
    async fn handle_request(&self, load_request: &LoadRequest) {
        let result = self.load_asset(load_request);
        let asset_result = AssetResult {
            handle: Handle::from(load_request.handle_id),
            result,
            path: load_request.path.clone(),
            version: load_request.version,
        };
        self.sender
            .send(asset_result)
            .expect("loaded asset should have been sent");
    }

    fn extensions(&self) -> &[&str] {
        self.loader.extensions()
    }
}
