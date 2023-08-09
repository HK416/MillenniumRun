mod bundle;
mod cache;
mod error;
mod handle;
mod integrity;
mod list;
mod path;
mod watcher;


pub use self::{
    bundle::AssetBundle,
    error::{AssetResult, AssetError, AssetErrorKind},
    handle::{AssetHandle, WeakAssetHandle},
    integrity::AssetDataType,
};


use self::{
    cache::AssetCache,
    integrity::AssetKeys,
    watcher::watcher_main,
    list::ASSET_LIST,
    path::RootAssetPath,
};


pub trait Asset : Default + Sized {
    fn decode_bytes(bytes: &[u8]) -> AssetResult<Self>;
    fn encode_bytes(&self) -> AssetResult<Vec<u8>>;
}
