// TODO: Path resolution
// 1. Look into the config file for paths
// 2. If some path is not specified in the config file check env and add that to the config file

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageKind {
    Config,
    Data,
}

#[derive(Debug, Clone)]
pub struct Paths {
    pub app_data_dir: std::path::PathBuf,
    pub app_config_dir: std::path::PathBuf,
}

impl Paths {
    #[must_use]
    pub fn new() -> Option<Self> {
        let app_data_dir = dirs::data_local_dir()?;
        let app_data_dir = app_data_dir.join(crate::APP_NAME);

        let app_config_dir = dirs::config_local_dir()?;
        let app_config_dir = app_config_dir.join(crate::APP_NAME);

        Some(Self {
            app_data_dir,
            app_config_dir,
        })
    }

    pub(super) fn resolve_path(
        &self,
        path_suffix: &str,
        storage_kind: StorageKind,
    ) -> std::path::PathBuf {
        match storage_kind {
            StorageKind::Config => self.app_config_dir.join(path_suffix),
            StorageKind::Data => self.app_data_dir.join(path_suffix),
        }
    }
}
