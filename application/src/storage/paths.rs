#[derive(Debug)]
pub struct Paths {
    pub app_data_dir: std::path::PathBuf,
    pub app_config_dir: std::path::PathBuf,
}

impl Paths {
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
}
