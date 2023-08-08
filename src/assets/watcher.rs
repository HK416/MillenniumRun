use std::path::Path;
use std::sync::mpsc;
use notify::{
    Event, 
    EventKind,
    event::ModifyKind,
    Config, 
    Watcher, 
    RecursiveMode, 
    RecommendedWatcher,
    Result as NotifyResult, 
};
use super::AssetCache;



pub fn watcher_main<P: AsRef<Path>>(asset_path: P, cache: AssetCache) -> NotifyResult<()> {
    log::debug!("watcher started.");
    let (sender, receiver) = mpsc::channel();
    
    let mut watcher = RecommendedWatcher::new(sender, Config::default())?;
    watcher.watch(asset_path.as_ref(), RecursiveMode::Recursive)?;

    for result in receiver {
        let event = result?;
        log::debug!("watcher event :: {:?} :: {:?}", event.kind, event.paths);
        if data_has_changed(&event) {
            for path in event.paths {
                if cache.get_handle(&path).is_some() {
                    log::warn!("something went wrong with the cached asset handle's data. :: {}", path.display());
                        cache.on_unsafety();
                        break;
                }
            }
        }
    }

    log::debug!("watcher finished.");
    Ok(())
}

#[inline]
fn data_has_changed(event: &Event) -> bool {
    if let EventKind::Modify(kind) = event.kind {
        if let ModifyKind::Data(_) = kind { return true; }
    }
    return false;
}
