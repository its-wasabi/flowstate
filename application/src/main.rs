#![allow(unused)]
#![allow(clippy::missing_errors_doc)]

pub mod storage;

const APP_NAME: &str = "flowstate";

#[derive(Debug)]
pub struct App {
    storage: storage::Storage,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let storage = storage::Storage::load_or_create()?;
        Ok(Self { storage })
    }
}

fn main() {
    let mut app = App::new().unwrap();
    println!("app: {app:#?}");

    app.storage.save();
}
