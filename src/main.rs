pub mod pocket;
pub mod tts;
pub mod c_new;

fn main() -> Result<(), anyhow::Error> {
    // tts::tts_speak();
    // pocket::pocket_reader();
    c_new::create();
    Ok(())
}
