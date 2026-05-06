#[derive(Debug)]
pub struct Paths {
    pub app_data_file: std::path::PathBuf,
    pub app_config_file: std::path::PathBuf,
}

impl Paths {
    pub fn new() -> Option<Self> {
        let app_data_file = dirs::data_local_dir()?;
        let app_data_file = app_data_file.join(crate::APP_NAME).join("flowstate.dat");

        let app_config_file = dirs::config_local_dir()?;
        let app_config_file = app_config_file.join(crate::APP_NAME).join("flowstate.json");

        Some(Self {
            app_data_file,
            app_config_file,
        })
    }
}
