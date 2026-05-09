use std::io::Write;

use application::App;
use automerge::{ObjId, transaction::Transactable};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new()?;

    _ = app
        .document
        .put_object(ObjId::Root, "x", automerge::ObjType::Map);

    let mut count = 0;
    loop {
        println!(" -={count}=- ");
        count += 1;

        _ = app.network.sync(&mut app.document);
        _ = app.storage.store(&mut app.document);
        std::thread::sleep(std::time::Duration::from_secs_f32(0.8));
    }
}
