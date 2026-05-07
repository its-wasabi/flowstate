use application::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new()?;
    println!("{app:#?}");
    Ok(())
}
